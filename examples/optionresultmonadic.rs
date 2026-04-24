  fn half_if_even(n: i32) -> Option<i32> {
      if n % 2 == 0 { Some(n / 2) } else { None }
  }

fn main() {

    let x = Some(20);

    // 성공하면 다음 closure 실행
    x.and_then(half_if_even);

    let f  = None;

    //실패하면 T를 반환 
    f.unwrap_or_else(|| {println!("it is fail")});
                          
    //실패하면 Option<T> 반환
    f.or_else(|| {println!("it is fail"); None});

}