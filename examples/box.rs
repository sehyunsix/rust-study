fn main () {

    let n : Box<i32> = Box::new(2);
    println!("{:?}",n);
    println!("{:p}", &n);
    println!("{:p}",&*n);
    


}