use num_cpus;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::key::{PublicKey, PrivateKey};
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;

pub fn introduction() {
    println!("\x1b[38;2;250;128;114m   ╔═════════════════════════════════════════════════╗");
    println!("\x1b[38;2;250;128;114m║\x1b[0m\x1b[1m\x1b[32m         KeryRypper v0.1.1 - Satoshi Quest            \x1b[0m\x1b[38;2;250;128;114m║");
    println!("\x1b[38;2;250;128;114m║\x1b[0m\x1b[1m\x1b[32m                    by Denzy Legacy                   \x1b[0m\x1b[38;2;250;128;114m║");
    println!("\x1b[38;2;250;128;114m   ╚═════════════════════════════════════════════════╝\x1b[0m");
}

pub fn machine_info() {
    let num_cpus = num_cpus::get();
    println!("\n[+] Host logical cores: {}", num_cpus);

    match sys_info::mem_info() {
        Ok(info) => {
            let total_gb = info.total as f64 / (1024.0 * 1024.0);
            let free_gb = info.free as f64 / (1024.0 * 1024.0);

            println!("[+] Total RAM: {:.2} GB, Free RAM: {:.2} GB", total_gb, free_gb);
        }
        Err(e) => {
            eprintln!("[+] Error retrieving RAM information: {}", e);
        }
    }

    let os_type = sys_info::os_type();
    let os_release = sys_info::os_release();

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
            let total_gb = disk.total as f64 / (1024.0 * 1024.0);
            let free_gb = disk.free as f64 / (1024.0 * 1024.0);

            println!("[+] Total disk space: {:.2} GB, Free: {:.2} GB\n", total_gb, free_gb);
        }
        Err(e) => {
            eprintln!("[+] Error retrieving disk information: {}\n", e);
        }
    }
}

pub fn generate_public_address(secp: &Secp256k1<bitcoin::secp256k1::All>, private_key_hex: &str) -> String {
    let private_key = PrivateKey::from_slice(&hex::decode(private_key_hex).unwrap(), Network::Bitcoin).unwrap();
    let public_key = PublicKey::from_private_key(secp, &private_key);
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    address.to_string()
}

pub fn generate_wif(private_key_hex: &str) -> String {
    let private_key = PrivateKey::from_slice(&hex::decode(private_key_hex).unwrap(), Network::Bitcoin).unwrap();
    private_key.to_wif()
}
