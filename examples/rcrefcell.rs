use std::cell::RefCell;
use std::rc::{self, Rc ,Weak};

#[derive(Debug)]
struct Data(i32);

impl Data  {    
    fn speak(&self){
        println!("speak");
    }
}

fn main(){


    let x = Data(1);
    let rc_x = Rc::new(RefCell::new(x));
    let rc_x_ref = rc_x.borrow_mut();
    
    
    let rc = Rc::new(Data(1));
    rc.speak();
    // println!("{}",rc.borrow());
    // Rc::downgrade(&rc);
}