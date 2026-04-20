fn main() {
    
  let v = vec![1, 2, 3];

  // safe
  println!("{}", v[1]);

  // unsafe
  let p = v.as_ptr();
  // unsafe {
  //     println!("{}", *p.add(1));
  // }  
  //  *p.add(1);

  unsafe {

    println!("{:?}",v);
  }
 
  
}
