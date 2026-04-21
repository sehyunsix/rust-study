use std::sync::Arc;
use std::thread;
use std::rc::Rc;


#[derive(Debug)]
struct Data { 
    value : i32
}

#[derive(Debug)]
struct  RcData {
    value : Rc<i32>
}

#[derive(Debug)]
struct  ArcData {
    value : Arc<i32>
}

fn main(){
    
    let mut  x: Data =  Data { value: 10 };
    let mut  y: RcData = RcData { value: Rc::new(10) };
    let mut  z: ArcData = ArcData { value: Arc::new(10) };
    
    // send + sync o 
    thread::spawn(move || { println!("{:?}",x)});
    
    // send + sync x

    // thread::spawn(move || { println!("{}",y.value)});

    thread::spawn(move || {println!("{:?}",z)});
}