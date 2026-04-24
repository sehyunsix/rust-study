  use std::sync::Arc;
  use std::sync::atomic::{AtomicUsize, Ordering};
  use std::thread;

  fn main() {
      let counter = Arc::new(AtomicUsize::new(0));
      let mut handles = vec![];

      for _ in 0..10 {
          let counter = Arc::clone(&counter);
          handles.push(thread::spawn(move || {
              counter.fetch_add(1, Ordering::SeqCst);
          }));
      }

      for h in handles {
          h.join().unwrap();
      }

      println!("{}", counter.load(Ordering::SeqCst));
  }