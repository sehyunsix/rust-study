use std::rc::Rc;
use std::rc::Weak;


fn main() {


    let foo = Rc::new(vec![1.0, 2.0, 3.0]);
    // The two syntaxes below are equivalent.
    let a = foo.clone();
    let b = Rc::clone(&foo);

    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",foo);

}