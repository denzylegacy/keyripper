// use std::thread;
use log::{error, info, warn};
use k256::elliptic_curve::FieldBytes;
use hex::FromHex;
use num_bigint::BigUint;
use num_traits::{Num, One};
extern crate secp256k1;
use secp256k1::constants;
use bitcoin::{Address, Network, PrivateKey, PublicKey};
use bitcoin::secp256k1::{All, Secp256k1, SecretKey};
use k256::{AffinePoint, EncodedPoint, ProjectivePoint, Scalar};
use k256::ecdsa::{SigningKey, VerifyingKey};

// use crate::services::bsgs::{BSGS, Point};

use std::collections::HashMap;
use hex;
use k256::elliptic_curve::group::GroupEncoding;
use k256::elliptic_curve::point::AffineCoordinates;
use k256::elliptic_curve::sec1::{FromEncodedPoint};
use libsecp256k1::curve::Field;
use num_traits::real::Real;

use crate::config::Config;
use crate::data::Address as TargetAddress;
use crate::services::key_search::math;
use crate::services::key_search::bsgs;

pub struct KeySearch {
    secp: Secp256k1<All>,
    curve: EllipticCurve,
}

#[derive(Debug)]
pub struct EllipticCurve {
    pub g: ProjectivePoint,
    pub order: [u8; 32],  // BigUint
}

impl KeySearch {

    pub fn new() -> Self {
        let curve = k256::Secp256k1::default();
        let g = ProjectivePoint::GENERATOR;
        let order = constants::CURVE_ORDER;

        let curve = EllipticCurve {
            g,
            order,
        };

        KeySearch {
            secp: Secp256k1::new(),
            curve,
        }
    }

    pub fn private_key_by_public_key(
        &self,
        hardware_info: &(),
        config: &Config,
        address: &TargetAddress,
    ) {
        // y^2 = x^3 + ax + b (mod p)
        let a = BigUint::from(0u32); // a = 0 -> SECP256k1
        let b = BigUint::from(7u32); // b = 7 -> SECP256k1
        let p = BigUint::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16
        ).unwrap();

        let public_key_x = BigUint::from_str_radix(
            &address.public_key_hex.as_str()[2..], 16)
            .expect("Error converting public_key_x to a whole number!");

        let mut y_square = (
            &public_key_x * &public_key_x * &public_key_x + &a * &public_key_x + &b
        ) % &p;

        let mut public_key_y = math::sqrt_mod_prime(&y_square, &p)
            .expect("Couldn't find a valid modular square root!");

        // Checks the public key prefix to choose the correct Y
        if (address.public_key_hex.as_str().starts_with("02") &&
            &public_key_y % 2u8 != BigUint::from(0u32)) ||
            (address.public_key_hex.as_str().starts_with("03") &&
                &public_key_y % 2u8 == BigUint::from(0u32)) {
            public_key_y = &p - &public_key_y;
        }

        // Public key point on the curve
        let x_bytes = public_key_x.to_bytes_be();
        let y_bytes = public_key_y.to_bytes_be();

        let mut encoded_point = Vec::with_capacity(65);
        encoded_point.push(0x04); // Uncompressed Prefix
        encoded_point.extend_from_slice(&x_bytes);
        encoded_point.extend_from_slice(&y_bytes);

        let encoded_point = EncodedPoint::from_bytes(&encoded_point)
            .expect("Failed to create EncodedPoint!");
        let target_public_key_point = ProjectivePoint::from_encoded_point(&encoded_point)
            .expect("Failed to create public key point");

        // Convert hex range to decimal
        let start_range = BigUint::from_str_radix(
            address.private_key_range_start.as_str(), 16
        ).expect("Invalid start range");

        let end_range = BigUint::from_str_radix(
            address.private_key_range_end.as_str(), 16
        ).expect("Invalid end range");

        let interval_size = &end_range - &start_range + BigUint::from(1u32);
        let max_steps = 2_usize.pow((interval_size.bits() / 2) as u32); // Calibrated max_steps

        let start_time = std::time::Instant::now();
        let mut total_steps_tried = 0;
        let mut private_key_integer = None;

        // Iterate over the range of private keys
        let mut current_start = start_range.clone();
        while current_start <= end_range {
            let key = bsgs::bsgs(
                &target_public_key_point, &ProjectivePoint::GENERATOR, &current_start, max_steps
            );
            total_steps_tried += max_steps;

            if let Some(found_key) = key {
                private_key_integer = Some(found_key);
                break;
            }

            println!("[+] Trying range: {} - {}", current_start, &current_start + max_steps);
            current_start += max_steps;
        }

        if let Some(key) = private_key_integer {
            let private_key_hex = format!("{:064x}", key);
            println!("Private key found: {}", private_key_hex);
            println!("WIF: {}", KeySearch::wif_by_private_key_hex(&private_key_hex));
            println!("Public address: {:?}", self.compressed_public_key_by_private_key_hex(&private_key_hex));
        } else {
            println!("Private key not found within the given range.");
        }

        println!("Elapsed time: {:?}", start_time.elapsed());
    }


    pub fn public_key_address_by_private_key_hex(
        secp: Secp256k1<All>,
        private_key_hex: &str,
    ) -> String {
        let private_key: PrivateKey =
            PrivateKey::from_slice(&hex::decode(private_key_hex).unwrap(), Network::Bitcoin).unwrap();
        let public_key: PublicKey = PublicKey::from_private_key(&secp, &private_key);
        let address: Address = Address::p2pkh(&public_key, Network::Bitcoin);
        address.to_string()
    }

    pub fn wif_by_private_key_hex(private_key_hex: &str) -> String {
        let private_key: PrivateKey =
            PrivateKey::from_slice(&hex::decode(private_key_hex).unwrap(), Network::Bitcoin).unwrap();
        private_key.to_wif()
    }

    pub fn compressed_public_key_by_private_key_hex(&self, private_key_hex: &str) -> Option<String> {
        if private_key_hex.is_empty() {
            error!("No private key hexadecimal was provided!");
            return None;
        }

        let private_key_bytes = Vec::from_hex(private_key_hex).ok()?;

        let private_key_field_bytes = FieldBytes::<k256::Secp256k1>::try_from(
            private_key_bytes.as_slice()
        ).ok()?;

        let signing_key = SigningKey::from_bytes(&private_key_field_bytes).ok()?;

        let verifying_key = VerifyingKey::from(&signing_key);

        let public_key_bytes = verifying_key.to_encoded_point(true).as_bytes().to_vec();
        let compressed_public_key_hex = hex::encode(public_key_bytes);

        // info!("Public Key (compressed): {}", compressed_public_key_hex);
        println!("Public Key (compressed): {}", compressed_public_key_hex);

        Some(compressed_public_key_hex)
    }
}
