use dotenv::dotenv;
use std::env;

pub struct Config {
    pub process: String,
    pub num_cores: usize,
    pub num_threads: usize,
    pub subrange_size: u64,
    pub server_url: String,
    pub api_auth_token: String
}

impl Config {
    pub fn load() -> Config {
        dotenv().ok();

        let process = env::var("PROCESS").unwrap_or_else(|_| "".to_string());

        let num_cores = env::var("NUM_CORES")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        let num_threads = env::var("NUM_THREADS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        let subrange_size = env::var("SUBRANGE_SIZE")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "".to_string());

        let api_auth_token = env::var("API_AUTH_TOKEN")
            .unwrap_or_else(|_| "".to_string());

        if !process.is_empty() {
            println!("[+] Mode: {:?}", process);
        }

        if num_cores != 0 {
            println!("[+] Logical Cores: {:?}", num_cores);
        }

        if num_threads != 0 {
            println!("[+] Threads: {:?}", num_threads);
        }

        if !process.is_empty() {
            println!("[+] Server URL: {:?}", server_url);
        }

        Config {
            process,
            num_threads,
            num_cores,
            subrange_size,
            server_url,
            api_auth_token
        }
    }
}
