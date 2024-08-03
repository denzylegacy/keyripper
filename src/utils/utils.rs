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
