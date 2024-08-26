#![allow(unused)]

use sys_info;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::data::Address;
use bitcoin::secp256k1::Secp256k1;
use num_cpus;

pub fn introduction() {
    println!("\x1b[38;2;250;128;114m   ╔═════════════════════════════════════════════════╗");
    println!("\x1b[38;2;250;128;114m║\x1b[0m\x1b[1m\x1b[32m          KeryRypper v0.2.0 - Satoshi Quest           \x1b[0m\x1b[38;2;250;128;114m║");
    println!("\x1b[38;2;250;128;114m║\x1b[0m\x1b[1m\x1b[32m                     by Denzy Legacy                  \x1b[0m\x1b[38;2;250;128;114m║");
    println!("\x1b[38;2;250;128;114m   ╚═════════════════════════════════════════════════╝\x1b[0m");
}

pub fn import_addresses(file_path: &str) -> Result<Vec<Address>, Box<dyn std::error::Error>> {
    let file = File::open(Path::new(file_path))?;
    let reader = BufReader::new(file);

    let addresses: Vec<Address> = serde_json::from_reader(reader)?;

    Ok(addresses)
}

#[derive(Debug)]
pub struct HardwareInfo {
    pub(crate) hostname: String,
    pub(crate) logical_cores: usize,
    pub(crate) current_processes: u64,
    pub(crate) cpu_speed_mhz: u64,
    pub(crate) cpu_speed_ghz: f64,
    pub(crate) total_ram_gb: f64,
    pub(crate) free_ram_gb: f64,
    pub(crate) os_type: String,
    pub(crate) os_release: String,
    pub(crate) total_disk_gb: f64,
    pub(crate) free_disk_gb: f64,
}

/// Gather host hardware information and return it as a `MachineInfo` object.
pub fn machine_info() -> Result<HardwareInfo, String> {

    let hostname = sys_info::hostname().map_err(
        |e| format!("Error retrieving host information: {}", e)
    )?;

    let logical_cores = num_cpus::get();

    let current_processes = sys_info::proc_total().map_err(
        |e| format!("Error retrieving processes information: {}", e)
    )?;

    let cpu_speed_mhz = sys_info::cpu_speed().map_err(
        |e| format!("Error retrieving CPU speed: {}", e)
    )?;
    let cpu_speed_ghz = cpu_speed_mhz as f64 / 1000.0;

    let mem_info = sys_info::mem_info().map_err(
        |e| format!("Error retrieving RAM information: {}", e)
    )?;
    let total_ram_gb = mem_info.total as f64 / (1024.0 * 1024.0);
    let free_ram_gb = mem_info.free as f64 / (1024.0 * 1024.0);

    let os_type = sys_info::os_type().map_err(
        |e| format!("Error retrieving operating system information: {}", e)
    )?;
    let os_release = sys_info::os_release().map_err(
        |e| format!("Error retrieving system version: {}", e)
    )?;

    let disk_info = sys_info::disk_info().map_err(
        |e| format!("Error retrieving disk information: {}", e)
    )?;
    let total_disk_gb = disk_info.total as f64 / (1024.0 * 1024.0);
    let free_disk_gb = disk_info.free as f64 / (1024.0 * 1024.0);

    Ok(HardwareInfo {
        hostname,
        logical_cores,
        current_processes,
        cpu_speed_mhz,
        cpu_speed_ghz,
        total_ram_gb,
        free_ram_gb,
        os_type,
        os_release,
        total_disk_gb,
        free_disk_gb,
    })
}

pub fn show_hardware_info(hardware: &HardwareInfo) {
    println!("[+] Hostname: {}", hardware.hostname);
    println!("[+] Logical Cores: {}", hardware.logical_cores);
    println!("[+] Current processes: {}", hardware.current_processes);
    println!(
        "[+] CPU Speed: {} MHz ({:.2} GHz)",
        hardware.cpu_speed_mhz, hardware.cpu_speed_ghz
    );
    println!(
        "[+] Total RAM: {:.2} GB ({:.2} GB free)",
        hardware.total_ram_gb, hardware.free_ram_gb
    );
    println!("[+] OS: {} v{}", hardware.os_type, hardware.os_release);
    println!(
        "[+] Total Disk Space: {:.2} GB ({:.2} GB free)\n",
        hardware.total_disk_gb, hardware.free_disk_gb
    );
}
