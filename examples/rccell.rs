use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

fn main(){


    let x= Rc::new(Cell::new(1));
    let y = Rc::clone(&x);
    
    
    println!("{:?}",x);
    println!("{:?}",y);
}