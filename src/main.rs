#![allow(unused)]

use secp256k1::{PublicKey, Secp256k1, SecretKey};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

mod key_search_approaches;
mod utils;

use key_search_approaches::ascending_search::ascending_search;
use utils::utils::generate_public_address;
use utils::utils::generate_wif;
use utils::utils::introduction;

fn main() {
    introduction();

    let min_key_range: u32 = 0x800;
    let max_key_range: u32 = 0xfff;
    let target_address: &str = "1DBaumZxUkM4qMQRt2LVWyFJq5kDtSZQot";

    let status_output_timer = 10u64;

    println!("[+] Loading...\n");

    println!("[+] Hunted address : {}", target_address);
    println!("[+] Initial hex    : {:#X}", min_key_range);
    println!("[+] Final hex      : {:#X}", max_key_range);
    println!("[+] Status output every {} secconds\n", status_output_timer);

    // ##### EXPERIMENTAL CODE #####

    let secp = Secp256k1::new();

    println!("***** STARTED **********");
    let current_time = Instant::now();

    let addresses: Vec<_> = (0..4_500)
        .map(|i| {
            let secret_key =
                SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
            let public_key = PublicKey::from_secret_key(&secp, &secret_key);
            public_key.serialize().to_vec()
        })
        .collect();

    let end_time = Instant::now();
    let total_time = end_time - current_time;
    println!("Generated {} keys", addresses.len());
    println!("[Runtime: {:#?}]", total_time);
}
