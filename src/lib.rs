use std::sync::{Arc, Mutex};
use std::thread;

/// Increments a counter by a given amount in a multi-threaded environment..
pub fn increment_counter(number: i32, by: i32, threads: u32) -> i32 {
    let shared = Arc::new(Mutex::new(number));

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                let mut num = shared.lock().unwrap();
                *num += by;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    *shared.lock().unwrap()
}
