use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

fn main () {
const N: usize = 10;

// Spawn a few threads to increment a shared variable (non-atomically), and
// let the main thread know once all increments are done.
//
// Here we're using an Arc to share memory among threads, and the data inside
// the Arc is protected with a mutex.
let mut n : i32 =0;
let data = Arc::new(n);

let (tx, rx) = channel();
for _ in 0..N {
    let (data, tx) = (Arc::clone(&data), tx.clone());
    thread::spawn(move || {
        // The shared state can only be accessed once the lock is held.
        // Our non-atomic increment is safe because we're the only thread
        // which can access the shared state when the lock is held.
        //
        // We unwrap() the return value to assert that we are not expecting
        // threads to ever fail while holding the lock.
        //data +=1 은 작동하지 않음
        // the lock is unlocked here when `data` goes out of scope.
    });
}

rx.recv().unwrap();

}