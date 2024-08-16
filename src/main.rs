#![deny(clippy::all)]
#![allow(unused)]

mod config;
mod engine;
mod utils;

use crate::utils::utils::machine_info;
use utils::utils::introduction;

use engine::keyripper::{KeySearch, EllipticCurve};
use engine::secp256k1_impl;

// #[tokio::main]
fn main() {
    env_logger::init();

    introduction();

    // let config: config::Config = config::Config::load();

    // let min_key_range = 0x80000;
    // let max_key_range = 0xfffff;
    // let target_address: &str = "1HsMJxNiV7TLxmoF6uJNkydxPFDog4NQum";

    let status_output_timer = 10u64;

    println!("[+] Loading...\n");

    // println!("[+] Hunted address : {}", target_address);
    // println!("[+] Initial hex    : {:#X}", min_key_range);
    // println!("[+] Final hex      : {:#X}", max_key_range);
    println!("[+] Status output every {} secconds", status_output_timer);

    machine_info();

    let key_search = KeySearch::new();

    // Private key to Public Key -> Ok
    // let private_key_hex = "00000000000000000000000000000000000000000000000000000000000d2c55";
    // if let Some(public_key) = key_search.compressed_public_key_by_private_key_hex(
    //     private_key_hex
    // ) { println!("{}", public_key) }
    // else {
    //     println!("Public key not found!");
    // }

    // Private Key by Public Key
    let public_key_hex = "033c4a45cbd643ff97d77f41ea37e843648d50fd894b864b0d52febc62f6454f7c";
    key_search.private_key_by_public_key(public_key_hex);
}
