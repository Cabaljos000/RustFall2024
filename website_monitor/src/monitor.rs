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

/// Function to perform a single website check.
fn check_website(url: &str, timeout: Duration) -> WebsiteStatus {
    let start_time = Instant::now();

    // Perform the HTTP request
    let response = ureq::get(url).timeout(timeout).call();

    let response_time = start_time.elapsed();
    let timestamp = Utc::now();

    // Handle the result of the response
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

/// Function to monitor websites with retries and worker pool.
pub fn monitor_websites(
    urls: Vec<String>,
    worker_count: usize,
    timeout: Duration,
    retries: u32,
) {
    let (sender, receiver) = mpsc::channel();
    let shared_urls = Arc::new(Mutex::new(urls)); // Wrap URLs in Arc<Mutex>

    // Clone sender before moving into the closure
    let sender_for_threads = sender.clone();

    // Spawn worker threads
    let thread_pool = WorkerPool::new(worker_count, move || {
        let thread_sender = sender_for_threads.clone(); // Clone sender for this thread
        let thread_urls = Arc::clone(&shared_urls); // Clone Arc for this thread

        loop {
            let url = {
                let mut urls = thread_urls.lock().unwrap();
                urls.pop() // This returns Option<String>
            };

            // Exit the loop if there are no more URLs
            if let Some(url) = url { // url is now a String
                let mut attempt = 0;
                while attempt <= retries {
                    let status = check_website(&url, timeout);

                    // Send the status to the receiver
                    if thread_sender.send(status.clone()).is_err() {
                        // If the receiver is dropped, stop sending
                        break;
                    }

                    // Break if successful
                    if status.status.is_ok() {
                        println!("Successfully fetched {} after {} attempts", url, attempt + 1);
                        break;
                    }

                    // Retry if necessary
                    attempt += 1;
                    println!("Retrying {} (attempt {}/{})", url, attempt, retries);
                }
            } else {
                // Exit loop if no URLs are left
                break;
            }
        }
    });

    // Original sender is dropped here
    drop(sender);

    // Receive and display results
    for status in receiver {
        println!("{:?}", status);
    }

    thread_pool.join_all();
}

