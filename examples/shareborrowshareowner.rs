use std::sync::Arc;

#[derive(Debug)]
struct  Data {
    value : i32
}

fn main (){
  let x = Data { value: 10 };
  let b = &x;
//   drop(x);
//   println!("{}", b.value); // 불가

  let x = Arc::new(Data {value :10 });
  let b = Arc::clone(&x);
  drop(x);
  println!("{:?}",b);
}