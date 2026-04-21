fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    //closure의 mut 은 closure를 캡처한 변수가 바뀔 수 있음을 뜻함.
    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}