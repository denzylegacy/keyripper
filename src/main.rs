use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::key::{PublicKey, PrivateKey};
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

fn generate_public(secp: &Secp256k1<bitcoin::secp256k1::All>, private_key_hex: &str) -> String {
    let private_key = PrivateKey::from_slice(&hex::decode(private_key_hex).unwrap(), Network::Bitcoin).unwrap();
    let public_key = PublicKey::from_private_key(secp, &private_key);
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    address.to_string()
}

fn generate_wif(private_key_hex: &str) -> String {
    let private_key = PrivateKey::from_slice(&hex::decode(private_key_hex).unwrap(), Network::Bitcoin).unwrap();
    private_key.to_wif()
}

fn find_private_key(
    min_range: u64,
    max_range: u64,
    target_address: &str,
    status_output_timer: u64,
    secp: Arc<Secp256k1<bitcoin::secp256k1::All>>,
    keys_checked: Arc<Mutex<u64>>,
    last_report_time: Arc<Mutex<Instant>>,
    keys_checked_in_interval: Arc<Mutex<u64>>,
) -> Option<String> {
    for private_key in min_range..=max_range {
        let private_key_hex = format!("{:064x}", private_key);
        let public_address = generate_public(&secp, &private_key_hex);

        // Updates the total number of verified keys
        {
            let mut checked = keys_checked.lock().unwrap();
            *checked += 1;
        }

        // Updates the number of keys scanned in the current interval
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

fn main() {

    println!("\x1b[38;2;250;128;114m   ╔═════════════════════════════════════════════════╗");
    println!("\x1b[38;2;250;128;114m║\x1b[0m\x1b[1m\x1b[32m         KeryRypper v0.1.0 - Satoshi Quest            \x1b[0m\x1b[38;2;250;128;114m║");
    println!("\x1b[38;2;250;128;114m║\x1b[0m\x1b[1m\x1b[32m                    by Denzy Legacy                   \x1b[0m\x1b[38;2;250;128;114m║");
    println!("\x1b[38;2;250;128;114m   ╚═════════════════════════════════════════════════╝\x1b[0m");


    let min_key_range = 0x80000;
    let max_key_range = 0xfffff;
    let target_address = "1HsMJxNiV7TLxmoF6uJNkydxPFDog4NQum";
    let status_output_timer = 10; 

    println!("[+] Loading...\n");

    println!("[+] Hunted address : {}", target_address);
    println!("[+] Initial hex    : {:#X}", min_key_range);
    println!("[+] Final hex      : {:#X}", max_key_range);
    println!("[+] Status output every {} secconds\n", status_output_timer);
    
    let secp = Arc::new(Secp256k1::new());
    let keys_checked = Arc::new(Mutex::new(0));
    let last_report_time = Arc::new(Mutex::new(Instant::now()));
    let keys_checked_in_interval = Arc::new(Mutex::new(0));

    let num_threads = 4;
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
            find_private_key(
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
            println!("[+] Public address: {}\n", generate_public(&secp, &private_key));
            return;
        }
    }

    println!("\x1b[38;2;250;128;114m\n[+] Private key not found in the given range!\n\x1b[0m");
}
