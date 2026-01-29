
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    Some(T),
    None,
}

impl Message {
    fn call(&self) {
        println!("Message called");
        // method body would be defined here
    }
  
}

fn main() {

  let home = IpAddr::V4(127,0,0,1);
  let loopback = IpAddr::V6(String::from("::1"));
  let m = Message::Write(String::from("hello"));
    m.call();   
    let x = 5;
    let y : Option<i32> = Option::None;
    let sum = match y {
        Option::Some(value) => x + value,
        Option::None => x,
    };
    println!("sum is {}", sum);
   
}
