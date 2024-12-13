use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use crate::worker_pool::WorkerPool;

/// Struct to hold the status of a website.
#[derive(Debug, Clone)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct MonitoringStats {
    pub avg_response_time: Duration,
    pub success_count: usize,
    pub failure_count: usize,
}


fn check_website(url: &str, timeout: Duration) -> WebsiteStatus {
    let start_time = Instant::now();

    let response = ureq::get(url).timeout(timeout).call();

    let response_time = start_time.elapsed();
    let timestamp = Utc::now();

    let status = match response {
        Ok(resp) => Ok(resp.status()),
        Err(err) => Err(format!("Failed to fetch {}: {}", url, err)),
    };

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time,
        timestamp,
    }
}

pub fn monitor_websites(
    urls: Vec<String>,
    worker_count: usize,
    timeout: Duration,
    retries: u32,
) {
    let (sender, receiver) = mpsc::channel();
    let shared_urls = Arc::new(Mutex::new(urls));

    let sender_for_threads = sender.clone();

    let thread_pool = WorkerPool::new(worker_count, move || {
        let thread_sender = sender_for_threads.clone();
        let thread_urls = Arc::clone(&shared_urls);

        loop {
            let url = {
                let mut urls = thread_urls.lock().unwrap();
                urls.pop()
            };

            if let Some(url) = url {
                let mut attempt = 0;
                while attempt <= retries {
                    let status = check_website(&url, timeout);

                    if thread_sender.send(status.clone()).is_err() {
                        break;
                    }

                    if status.status.is_ok() {
                        println!("Successfully fetched {} after {} attempt(s)", url, attempt + 1);
                        break;
                    }

                    attempt += 1;
                    println!("Retrying {} (attempt {}/{})", url, attempt, retries);
                }
            } else {
                break;
            }
        }
    });

    drop(sender);

    let mut stats = MonitoringStats::default();
    let mut total_response_time = Duration::ZERO;

    for status in receiver {
        println!("{:?}", status);
        match status.status {
            Ok(_) => stats.success_count += 1,
            Err(_) => stats.failure_count += 1,
        }
        total_response_time += status.response_time;
    }

    stats.avg_response_time = if stats.success_count > 0 {
        total_response_time / stats.success_count as u32
    } else {
        Duration::ZERO
    };

    println!("Monitoring Statistics: {:?}", stats);

    thread_pool.join_all();
}

