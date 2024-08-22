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

        Config {
            process,
            num_threads,
            num_cores,
        }
    }
}
