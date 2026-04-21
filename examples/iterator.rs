

fn main() {

    let data = [1,2,3,4,5,6,7,8,9,10];
    
    for e in data.iter() {
        print!("{} ", e);
    }
  
    println!();

    //map
    for e in data.iter().map(|x| x+1) {
            print!("{} ", e);
    }

    println!();
    
    //filter
    //iter() -> &i32 filter -> &&i32 따라서 &를 해제해구나, deref 해주어야함.
    for e in data.iter().filter(|&x| *x>5 ){
            print!("{} ", e);
    }

    println!();

    //fold
    // 누적 연산을 할떄 사용. 
    let mul = data.iter().fold(1, |acc ,x| acc * x);
    print!("{} ",mul);

}