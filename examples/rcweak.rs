  use std::rc::{Rc, Weak};

  fn main() {
      let strong = Rc::new(String::from("hello"));

      let weak: Weak<String> = Rc::downgrade(&strong);

      println!("strong = {}", Rc::strong_count(&strong)); // 1
      println!("weak = {}", Rc::weak_count(&strong));     // 1

      if let Some(value) = weak.upgrade() {
          println!("upgraded: {}", value);
      }
  }