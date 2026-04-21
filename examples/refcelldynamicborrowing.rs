use std::cell::RefCell;
fn main(){


//  let mut x = 10;
//  let a = &x;
//  let b = &mut x; // 컴파일 에러
//  println!("{}",a);

 let x = RefCell::new(10);

  
  let mut v = x.borrow_mut();
 *v += 1;
   // 여기서 mutable borrow 종료

  let y = x.borrow();
  println!("{}", *y);


    
}