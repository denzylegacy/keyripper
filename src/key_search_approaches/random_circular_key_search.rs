#![allow(unused)]

use bitcoin::secp256k1::Secp256k1;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::utils::utils::generate_public_address;

pub fn random_circular_key_search(
    min_range: u128,
    max_range: u128,
    target_address: &str,
    status_output_timer: u64,
    secp: Arc<Secp256k1<bitcoin::secp256k1::All>>,
    keys_checked: Arc<Mutex<u64>>,
    last_report_time: Arc<Mutex<Instant>>,
    keys_checked_in_interval: Arc<Mutex<u64>>,
) -> Option<String> {
    let mut rng = rand::thread_rng();

    None
}
