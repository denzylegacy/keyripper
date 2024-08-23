use dotenv::dotenv;
use std::env;

pub struct Config {
    pub process: String,
    pub num_cores: u8,
    pub num_threads: u8,
}

impl Config {
    pub fn load() -> Config {
        dotenv().ok();

        let process = env::var("PROCESS").unwrap_or_else(|_| "".to_string());

        let num_cores = env::var("NUM_CORES")
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(0);

        let num_threads = env::var("NUM_THREADS")
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(0);

        if !process.is_empty() {
            println!("[+] Mode: {:?}", process);
        }

        if num_cores != 0 {
            println!("[+] Logical Cores: {:?}", num_cores);
        }

        if num_threads != 0 {
            println!("[+] Threads: {:?}", num_threads);
        }

        Config {
            process,
            num_threads,
            num_cores,
        }
    }
}
