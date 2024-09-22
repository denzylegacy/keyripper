use log::{error, info, warn};
use k256::elliptic_curve::FieldBytes;
use hex::FromHex;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{Num, One, ToPrimitive};
extern crate secp256k1;
use secp256k1::constants;
use bitcoin::{Address, Network, PrivateKey, PublicKey};
use bitcoin::secp256k1::{All, Secp256k1, SecretKey};
use k256::{AffinePoint, EncodedPoint, ProjectivePoint, Scalar};
use k256::ecdsa::{SigningKey, VerifyingKey};
use std::collections::HashMap;
use std::error::Error;
use hex;
use k256::elliptic_curve::group::GroupEncoding;
use k256::elliptic_curve::point::AffineCoordinates;
use k256::elliptic_curve::sec1::{FromEncodedPoint};
use libsecp256k1::curve::Field;
use num_traits::real::Real;
use crate::utils::utils::{HardwareInfo};
use crate::config::Config;
use crate::data::Address as TargetAddress;
use crate::services::key_search::math;
use crate::services::key_search::bsgs;

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use rand::Rng;
use reqwest::Client;
use serde::Serialize;
use tokio::runtime::Runtime;

pub struct KeySearch {
    secp: Secp256k1<All>,
    curve: EllipticCurve,
}

#[derive(Debug)]
pub struct EllipticCurve {
    pub g: ProjectivePoint,
    pub order: [u8; 32],  // BigUint
}

#[derive(Serialize)]
pub struct Payload {
    pub _bit_range: String,
    pub _private_key_hex: String,
    pub _wif: String,
    pub _public_address: String,
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
        hardware_info: &HardwareInfo,
        config: &Config,
        address: &TargetAddress,
    ) {
        let start_time = std::time::Instant::now();

        // Elliptic Curve Configuration SECP256k1
        let a = BigUint::from(0u32);
        let b = BigUint::from(7u32);
        let p = BigUint::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16
        ).unwrap();

        // Public key recovery
        let public_key_x = BigUint::from_str_radix(
            &address.public_key_hex.as_str()[2..], 16
        ).expect("Error converting public_key_x to whole number!");

        let mut y_square = (
            &public_key_x * &public_key_x * &public_key_x + &a * &public_key_x + &b
        ) % &p;

        let mut public_key_y = math::sqrt_mod_prime(&y_square, &p)
            .expect("Couldn't find a valid modular square root!");

        // Public Key Prefix Verification
        if (address.public_key_hex.as_str().starts_with("02") &&
            &public_key_y % 2u8 != BigUint::from(0u32)) ||
            (address.public_key_hex.as_str().starts_with("03") &&
                &public_key_y % 2u8 == BigUint::from(0u32)) {
            public_key_y = &p - &public_key_y;
        }

        // Creating the public key point on the curve
        let x_bytes = public_key_x.to_bytes_be();
        let y_bytes = public_key_y.to_bytes_be();

        let mut encoded_point = Vec::with_capacity(65);
        encoded_point.push(0x04); // Uncompressed Prefix
        encoded_point.extend_from_slice(&x_bytes);
        encoded_point.extend_from_slice(&y_bytes);

        let encoded_point = EncodedPoint::from_bytes(&encoded_point)
            .expect("Failed to create EncodedPoint");
        let target_public_key_point = ProjectivePoint::from_encoded_point(&encoded_point)
            .expect("Failed to create public key point");

        // Converting the hexadecimal range to decimal
        let start_range = BigUint::from_str_radix(
            address.private_key_range_start.as_str(), 16
        ).expect("Invalid Start Range");

        let end_range = BigUint::from_str_radix(
            address.private_key_range_end.as_str(), 16
        ).expect("Invalid End Range");

        let total_range = &end_range - &start_range + BigUint::one();

        // Subrange Size
        let subrange_size = BigUint::from(100_000_000_000u64);

        let current_position = Arc::new(Mutex::new(start_range.clone()));
        let target_public_key_point = Arc::new(target_public_key_point);
        let total_steps_tried = Arc::new(Mutex::new(0usize));
        let private_key_integer = Arc::new(Mutex::new(None));

        let (tx, rx) = mpsc::channel();
        let mut threads = vec![];

        for _ in 0..config.num_threads {
            let tx = tx.clone();
            let current_position = Arc::clone(&current_position);
            let end_range = end_range.clone();
            let subrange_size = subrange_size.clone();
            let target_public_key_point = Arc::clone(&target_public_key_point);
            let total_steps_tried = Arc::clone(&total_steps_tried);
            let private_key_integer = Arc::clone(&private_key_integer);

            let thread = thread::spawn(move || {
                loop {
                    {
                        if private_key_integer.lock().unwrap().is_some() {
                            break;
                        }
                    }

                    let (current_start, current_end) = {
                        let mut pos = current_position.lock().unwrap();
                        if *pos > end_range {
                            break;
                        }

                        let current_start = pos.clone();
                        let potential_end = &current_start + &subrange_size - BigUint::one();

                        let current_end = if potential_end > end_range {
                            end_range.clone()
                        } else {
                            potential_end
                        };

                        *pos = &current_end + BigUint::one();

                        (current_start, current_end)
                    };

                    let interval_size = &current_end - &current_start + BigUint::one();
                    let max_steps = (interval_size.sqrt() + BigUint::one())
                        .to_usize()
                        .unwrap_or(std::usize::MAX);

                    println!(
                        "[+] Thread {:?} searching: {} - {}",
                        thread::current().id(), current_start, current_end
                    );

                    let key = bsgs::bsgs(
                        &target_public_key_point,
                        &ProjectivePoint::GENERATOR,
                        &current_start,
                        max_steps,
                    );

                    {
                        let mut steps = total_steps_tried.lock().unwrap();
                        *steps += max_steps;
                    }

                    if let Some(found_key) = key {
                        {
                            let mut private_key = private_key_integer.lock().unwrap();
                            *private_key = Some(found_key.clone());
                        }
                        tx.send(found_key.clone()).unwrap();
                        break;
                    }
                }
            });
            threads.push(thread);
        }

        drop(tx);

        if let Ok(key) = rx.recv() {
            let private_key_hex = format!("{:064x}", key);

            println!("\nPrivate Key Found! <{}>", private_key_hex);

            let payload = Payload {
                _bit_range: (&address.bit_range.as_str()).parse().unwrap(),
                _private_key_hex: private_key_hex.clone(),
                _wif: KeySearch::wif_by_private_key_hex(&private_key_hex),
                _public_address: self.compressed_public_key_by_private_key_hex(
                    &private_key_hex).unwrap().to_string(),
            };

            if let Err(e) = self.server_bridge(
                &config.server_url, &config.api_auth_token, &payload) {
                eprintln!("Failed to send the data: {}", e);
            } else {
                println!("Data successfully sent to the server.");
            }

        } else {
            println!("Private key not found within the given range.");
        }

        for thread in threads {
            thread.join().unwrap();
        }

        println!("Elapsed time: {:?}", start_time.elapsed());
        println!("Total steps attempted: {}", *total_steps_tried.lock().unwrap());
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

        Some(compressed_public_key_hex)
    }

    pub fn server_bridge(
        &self,
        url: &str,
        token: &str,
        payload: &Payload,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let client = Client::new();
        let rt = Runtime::new()?;

        let response = rt.block_on(async {
            client.post(url)
                .json(&payload)
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
        })?;

        Ok(response)
    }
}
