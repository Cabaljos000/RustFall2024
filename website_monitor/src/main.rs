mod monitor;
mod worker_pool;

use std::time::Duration;

fn main() {
    let urls = vec![
        "https://www.rust-lang.org".to_string(),
        "https://www.google.com".to_string(),
        "https://www.github.com".to_string(),
    ];

    let worker_count = 2; // Number of threads
    let timeout = Duration::from_secs(5); // Timeout for requests
    let retries = 3; // Max retries per website

    monitor::monitor_websites(urls, worker_count, timeout, retries);
}
