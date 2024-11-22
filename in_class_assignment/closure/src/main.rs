use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    cached_result: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            cached_result: None,
        }
    }

    fn get_result(&mut self) -> String {
        // Check if result is already cached
        if let Some(ref result) = self.cached_result {
            return result.clone();
        }

        // Compute the result, cache it, and return
        let result = (self.computation)();
        self.cached_result = Some(result.clone());
        result
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}
