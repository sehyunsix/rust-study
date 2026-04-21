use std::thread;

#[derive(Debug)]
struct Data {
    value : i32
}

fn main(){


    let x : Data = Data { value: 4 } ; 
    let mut p : *const Data = &x;
    drop(x);
    
    // p1은 값에 접근하기 위해서 unsafe가 필요함.
    // println!("{:?}" ,*p);

    unsafe  {
        println!("{:?}" ,*p);
    }

    // p1는 raw pointer이므로 thread에서 move할 수 없음. (소유권 복사 불가)   
    // thread::spawn(move || {
    //     println!("{:?}", p);
    // });
    
    println!("{:?}", p);
    // println!("{:?}", x);

}