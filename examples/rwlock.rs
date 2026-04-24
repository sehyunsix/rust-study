use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main(){
    let data = Arc::new(RwLock::new(5));

    // Multiple readers can access in parallel.
    for i in 0..3 {
        let lock_clone = Arc::clone(&data);

        thread::spawn(move || {
            let value = lock_clone.read().unwrap();

            println!("Reader {}: Read value {}, now holding lock...", i, *value);

            // Simulating a long read operation
            thread::sleep(Duration::from_secs(1));

            println!("Reader {}: Dropping lock.", i);
            // Read lock unlocked when going out of scope.
        });
    }

    thread::sleep(Duration::from_millis(100));  // Wait for readers to start

    // While all readers can proceed, a call to .write() has to wait for
    let mut writable_data = data.write().unwrap();
    println!("Writer proceeds...");
    *writable_data += 1;

}