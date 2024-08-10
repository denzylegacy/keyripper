use bitcoin::secp256k1::Secp256k1;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

mod config;
mod key_search_approaches;
mod utils;

use crate::utils::utils::machine_info;
use utils::utils::generate_public_address;
use utils::utils::generate_wif;
use utils::utils::introduction;

use key_search_approaches::ascending_search::ascending_search;
use key_search_approaches::random_circular_key_search::random_circular_key_search;

fn main() {
    introduction();

    let config: config::Config = config::Config::load();

    let min_key_range = 0x20000000000000000;
    let max_key_range = 0x3ffffffffffffffff;
    let target_address: &str = "1DBaumZxUkM4qMQRt2LVWyFJq5kDtSZQot";

    let status_output_timer = 10u64;

    println!("[+] Loading...\n");

    println!("[+] Hunted address : {}", target_address);
    println!("[+] Initial hex    : {:#X}", min_key_range);
    println!("[+] Final hex      : {:#X}", max_key_range);
    println!("[+] Status output every {} secconds", status_output_timer);

    machine_info();

    let secp: Arc<Secp256k1<bitcoin::secp256k1::All>> = Arc::new(Secp256k1::new());
    let keys_checked: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    let last_report_time: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    let keys_checked_in_interval: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let num_threads = 4u128;
    let range_per_thread: u128 = (max_key_range - min_key_range + 1) / num_threads;

    let mut handles: Vec<thread::JoinHandle<Option<String>>> = vec![];

    for i in 0..config.num_threads {
        let secp_clone: Arc<Secp256k1<bitcoin::secp256k1::All>> = Arc::clone(&secp);
        let keys_checked_clone: Arc<Mutex<u64>> = Arc::clone(&keys_checked);
        let last_report_time_clone: Arc<Mutex<Instant>> = Arc::clone(&last_report_time);
        let keys_checked_in_interval_clone: Arc<Mutex<u64>> = Arc::clone(&keys_checked_in_interval);
        let start: u128 = min_key_range + i * range_per_thread;

        let end: u128 = if i == config.num_threads - 1 {
            max_key_range
        } else {
            start + range_per_thread - 1
        };

        let handle: thread::JoinHandle<Option<String>> = thread::spawn(move || {
            if config.search_approach == 0 {
                ascending_search(
                    start,
                    end,
                    target_address,
                    status_output_timer,
                    secp_clone,
                    keys_checked_clone,
                    last_report_time_clone,
                    keys_checked_in_interval_clone,
                )
            } else if config.search_approach == 1 {
                random_circular_key_search(
                    start,
                    end,
                    target_address,
                    status_output_timer,
                    secp_clone,
                    keys_checked_clone,
                    last_report_time_clone,
                    keys_checked_in_interval_clone,
                )
            } else {
                None
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Some(private_key) = handle.join().unwrap() {
            println!("\n[+] Private key found: {}", private_key);
            println!(
                "\x1b[1m\x1b[32m[+] WIF\x1b[0m: {}",
                generate_wif(&private_key)
            );
            println!(
                "[+] Public address: {}\n",
                generate_public_address(&secp, &private_key)
            );
            return;
        }
    }

    println!("\x1b[38;2;250;128;114m\n[+] Private key not found in the given range!\n\x1b[0m");
}
