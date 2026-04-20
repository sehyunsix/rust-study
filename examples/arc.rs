use std::sync::Arc;
  use std::thread;
use std::time::Duration;

  fn main() {
      let n = Arc::new(5);

      for _ in 0..3 {
          let n = Arc::clone(&n);

          //새로운 thread가 오래 살아남을 수 없어서 , n의 소유권을 가져감.
          //move 를 안하면 바깥변수에서 참조만 가져감.
          //send + sync 일때만 안전함.
          //여기서 send 란 T 값을 다른 쓰레드로 move 할 수 있음을 의미함.
          //Arc<T> 는 T가 send + sync이면 , send + sync임.
          
          thread::spawn(move || {
              println!("{n}");
          });
      }
      
      thread::sleep(Duration::from_micros(10_));
  }