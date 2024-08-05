use dotenv::dotenv;
use std::env;

pub struct Config {
    pub num_threads: u128,
    pub search_approach: u32,
    // pub environment: String,
}

impl Config {
    pub fn load() -> Config {
        dotenv().ok();

        let num_threads = env::var("NUM_THREADS")
            .ok()
            .and_then(|v| v.parse::<u128>().ok())
            .unwrap_or(1);
        let search_approach = env::var("SEARCH_APPROACH")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0);
        // let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "".to_string());

        Config {
            num_threads,
            search_approach,
            // environment,
        }
    }
}
