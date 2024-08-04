use bitcoin::secp256k1::Secp256k1;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

use crate::utils::utils::generate_public_address;

pub fn ascending_search(
    min_range: u128,
    max_range: u128,
    target_address: &str,
    status_output_timer: u64,
    secp: Arc<Secp256k1<bitcoin::secp256k1::All>>,
    keys_checked: Arc<Mutex<u64>>,
    last_report_time: Arc<Mutex<Instant>>,
    keys_checked_in_interval: Arc<Mutex<u64>>,
) -> Option<String> {
    for private_key in min_range..=max_range {
        let private_key_hex = format!("{:064x}", private_key);
        let public_address = generate_public_address(&secp, &private_key_hex);
        
        {
            let mut checked = keys_checked.lock().unwrap();
            *checked += 1;
        }

        {
            let mut interval_checked = keys_checked_in_interval.lock().unwrap();
            *interval_checked += 1;
        }
        
        let current_time = Instant::now();
        {
            let mut last_time = last_report_time.lock().unwrap();
            if current_time.duration_since(*last_time) >= Duration::from_secs(status_output_timer) {
                let checked = keys_checked.lock().unwrap();
                let mut interval_checked = keys_checked_in_interval.lock().unwrap();

                let average_keys_per_second = *interval_checked as f64 / status_output_timer as f64;

                println!("[+] {} computed keys: ~{:.2} keys/s", *checked, average_keys_per_second);
                
                *interval_checked = 0;

                *last_time = current_time;
            }
        }
        
        if public_address == target_address {
            return Some(private_key_hex);
        }
    }

    None
}
