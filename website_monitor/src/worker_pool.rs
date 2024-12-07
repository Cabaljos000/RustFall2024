use std::sync::{Arc};
use std::thread;

pub struct WorkerPool {
    workers: Vec<thread::JoinHandle<()>>,
}

impl WorkerPool {
    /// Create a new WorkerPool.
    pub fn new<F>(num_workers: usize, task: F) -> Self
    where
        F: Fn() + Send + 'static + Clone + std::marker::Sync,
    {
        let task = Arc::new(task);
        let workers = (0..num_workers)
            .map(|_| {
                let task = Arc::clone(&task);
                thread::spawn(move || {
                    task();
                })
            })
            .collect();

        WorkerPool { workers }
    }

    /// Join all worker threads.
    pub fn join_all(self) {
        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}
