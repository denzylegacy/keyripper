<<<<<<< Updated upstream
use bitcoin::secp256k1::Secp256k1;
use std::time::{Instant};
=======
#![allow(unused)]

use secp256k1::{PublicKey, Secp256k1, SecretKey};

// use bitcoin::secp256k1::Secp256k1;
>>>>>>> Stashed changes
use std::sync::{Arc, Mutex};
use std::thread;

mod utils;
mod key_search_approaches;

use utils::utils::introduction;
use utils::utils::generate_public_address;
use utils::utils::generate_wif;
use key_search_approaches::ascending_search::ascending_search;

fn main() {
    introduction();

<<<<<<< Updated upstream
    let min_key_range: u128 = 0x20000000000000000;
    let max_key_range: u128 = 0x3ffffffffffffffff;
    let target_address = "13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so";
    let status_output_timer = 10;
=======
    let min_key_range: u32 = 0x800;
    let max_key_range: u32 = 0xfff;
    let target_address: &str = "1DBaumZxUkM4qMQRt2LVWyFJq5kDtSZQot";

    let status_output_timer = 10u64;
>>>>>>> Stashed changes

    println!("[+] Loading...\n");

    println!("[+] Hunted address : {}", target_address);
    println!("[+] Initial hex    : {:#X}", min_key_range);
    println!("[+] Final hex      : {:#X}", max_key_range);
    println!("[+] Status output every {} secconds\n", status_output_timer);
    
    let secp = Arc::new(Secp256k1::new());
    let keys_checked = Arc::new(Mutex::new(0));
    let last_report_time = Arc::new(Mutex::new(Instant::now()));
    let keys_checked_in_interval = Arc::new(Mutex::new(0));

<<<<<<< Updated upstream
    let num_threads = 8;
    let range_per_thread = (max_key_range - min_key_range + 1) / num_threads;

    let mut handles = vec![];

    for i in 0..num_threads {
        let secp_clone = Arc::clone(&secp);
        let keys_checked_clone = Arc::clone(&keys_checked);
        let last_report_time_clone = Arc::clone(&last_report_time);
        let keys_checked_in_interval_clone = Arc::clone(&keys_checked_in_interval);
        let start = min_key_range + i * range_per_thread;
        
        let end = if i == num_threads - 1 {
            max_key_range
        } else {
            start + range_per_thread - 1
        };

        let handle = thread::spawn(move || {
            ascending_search(
                start, end, 
                target_address, 
                status_output_timer, 
                secp_clone, 
                keys_checked_clone, 
                last_report_time_clone, 
                keys_checked_in_interval_clone
            )
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Some(private_key) = handle.join().unwrap() {
            println!("\n[+] Private key found: {}", private_key);
            println!("\x1b[1m\x1b[32m[+] WIF\x1b[0m: {}", generate_wif(&private_key));
            println!("[+] Public address: {}\n", generate_public_address(&secp, &private_key));
            return;
        }
    }

    println!("\x1b[38;2;250;128;114m\n[+] Private key not found in the given range!\n\x1b[0m");
=======
    //let secp: Arc<Secp256k1<bitcoin::secp256k1::All>> = Arc::new(Secp256k1::new());
    let keys_checked: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    let last_report_time: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    let keys_checked_in_interval: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let num_threads = 4u128;

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
>>>>>>> Stashed changes
}
