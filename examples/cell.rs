use std::{cell::Cell};

  fn main() {
      let x = Cell::new(10);

      println!("{}", x.get());
      x.set(20);
      println!("{}", x.get());


      
    }