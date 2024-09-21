#![deny(clippy::all)]
#![allow(unused)]

mod config;
mod services;
mod utils;
mod data;

use crate::utils::utils::{machine_info, import_addresses, show_hardware_info, HardwareInfo};
use utils::utils::introduction;

use services::key_search::keyripper::{KeySearch, EllipticCurve};
use crate::config::Config;
use crate::data::Address;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    introduction();

    let hardware_info = match machine_info() {
        Ok(hardware) => {
            show_hardware_info(&hardware);
            hardware
        }
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    println!("[+] Preconfigured Processes");

    let config = Config::load();

    let addresses = import_addresses("./src/data/addresses.json")?;

    match config.process.as_str() {
        "SEARCH_PRIV_KEY_BY_ADDR" => search_private_key_by_address(&addresses),
        "SEARCH_PUB_KEY" => search_public_key_by_private_key(&addresses),
        _ => search_private_key_by_public_key(&hardware_info, config, &addresses),
    }

    Ok(())
}

/// Executes the process of searching for a private key by a public key.
///
/// This process uses the `KeySearch` class to find the private key
/// corresponding to a given public key.
fn search_private_key_by_public_key(
    hardware_info: &HardwareInfo,
    config: Config,
    addresses: &Vec<Address>
) {
    for i in (5..=addresses.len()).step_by(5) {
        if let Some(address) = addresses.get(i - 1) {
            if !address.solved {
                println!("\n[+] Activating Private Key from Public Key search");
                println!("[+] Address: {:?}: {}", address.address, address.bit_range);

                /// Divide the processor by the core numbers and the key range

                println!("\n&hardware_info.logical_cores {:?}", hardware_info.logical_cores);
                println!("&config.num_cores {:?}\n", config.num_cores);
                println!("&config.num_threads {:?}\n", config.num_threads);

                let key_search = KeySearch::new();

                key_search.private_key_by_public_key(&hardware_info, &config, &address);

                break;
            }
        }
    }
}

/// Executes the process of searching for a public key by a private key.
///
/// This process uses the `KeySearch` class to find the public key
/// corresponding to a given private key. If the public key is not
/// found, an error message will be displayed.
fn search_public_key_by_private_key(addresses: &Vec<Address>) {
    for i in 1..=addresses.len() {
        if let Some(address) = addresses.get(i) {
            if !address.solved {
                println!("\n[+] {:?}: {}", address.address, address.bit_range);

                let key_search = KeySearch::new();

                if let Some(public_key) = key_search.compressed_public_key_by_private_key_hex(
                    address.private_key_hex.as_str()
                ) {
                    println!("{}", public_key);
                } else {
                    println!("Public key not found!");
                }
            }
        }
    }
}

fn search_private_key_by_address(addresses: &Vec<Address>) {
    let status_output_timer = 10u64;
    println!("\n[+] Status output every {} seconds", status_output_timer);
}
