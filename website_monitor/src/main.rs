mod monitor;
mod worker_pool;

use std::time::Duration;
use std::thread;
use std::fs;

fn main() {
    let urls: Vec<_> = match fs::read_to_string("websites.txt") {
        Ok(content) => content.lines().map(|line| line.to_string()).collect(),
        Err(err) => {
            eprintln!("Failed to read websites.txt: {}", err);
            return;
        }
    };

    let worker_count = 10; // Increased for 50+ URL testing
    let timeout = Duration::from_secs(5); // Timeout for requests
    let retries = 3; // Max retries per website
    let interval = Duration::from_secs(30); // Periodic monitoring interval

    // Periodic monitoring
    loop {
        monitor::monitor_websites(urls.clone(), worker_count, timeout, retries);
        println!("Sleeping for {:?} before the next monitoring cycle", interval);
        thread::sleep(interval);
    }
}