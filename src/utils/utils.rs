#![allow(unused)]

use bitcoin::address::Address;
use bitcoin::key::{PrivateKey, PublicKey};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;
use num_cpus;

pub fn introduction() {
    println!("\x1b[38;2;250;128;114m  ╔════════════════════════════════════════════════╗");
    println!("\x1b[38;2;250;128;114m║\x1b[0m\x1b[1m\x1b[32m          KeryRypper v0.1.2 - Satoshi Quest         \x1b[0m\x1b[38;2;250;128;114m║");
    println!("\x1b[38;2;250;128;114m║\x1b[0m\x1b[1m\x1b[32m                   by Denzy Legacy                  \x1b[0m\x1b[38;2;250;128;114m║");
    println!("\x1b[38;2;250;128;114m  ╚════════════════════════════════════════════════╝\x1b[0m");
}

pub fn machine_info() {
    let num_cpus: usize = num_cpus::get();
    println!("\n[+] Host logical cores: {}", num_cpus);

    match sys_info::mem_info() {
        Ok(info) => {
            let total_gb: f64 = info.total as f64 / (1024.0 * 1024.0);
            let free_gb: f64 = info.free as f64 / (1024.0 * 1024.0);

            println!(
                "[+] Total RAM: {:.2} GB, Free RAM: {:.2} GB",
                total_gb, free_gb
            );
        }
        Err(e) => {
            eprintln!("[+] Error retrieving RAM information: {}", e);
        }
    }

    let os_type: Result<String, sys_info::Error> = sys_info::os_type();
    let os_release: Result<String, sys_info::Error> = sys_info::os_release();

    match (os_type, os_release) {
        (Ok(os), Ok(release)) => {
            println!("[+] OS: {} v{}", os, release);
        }
        (Err(e), _) => {
            eprintln!("[+] Error retrieving operating system information: {}", e);
        }
        (_, Err(e)) => {
            eprintln!("[+] Error retrieving system version: {}", e);
        }
    }

    match sys_info::disk_info() {
        Ok(disk) => {
            let total_gb: f64 = disk.total as f64 / (1024.0 * 1024.0);
            let free_gb: f64 = disk.free as f64 / (1024.0 * 1024.0);

            println!(
                "[+] Total disk space: {:.2} GB, Free: {:.2} GB\n",
                total_gb, free_gb
            );
        }
        Err(e) => {
            eprintln!("[+] Error retrieving disk information: {}\n", e);
        }
    }
}

pub fn generate_public_address(
    secp: &Secp256k1<bitcoin::secp256k1::All>,
    private_key_hex: &str,
) -> String {
    let private_key_bytes: Vec<u8> = hex::decode(private_key_hex).expect("Invalid hex");
    let private_key: PrivateKey =
        PrivateKey::from_slice(&private_key_bytes, Network::Bitcoin).expect("Invalid private key");
    let public_key: PublicKey = PublicKey::from_private_key(secp, &private_key);
    let address: Address = Address::p2pkh(&public_key, Network::Bitcoin);
    address.to_string()
}

pub fn generate_wif(private_key_hex: &str) -> String {
    let private_key_bytes: Vec<u8> = hex::decode(private_key_hex).expect("Invalid hex");
    let private_key: PrivateKey =
        PrivateKey::from_slice(&private_key_bytes, Network::Bitcoin).expect("Invalid private key");
    private_key.to_wif()
}
