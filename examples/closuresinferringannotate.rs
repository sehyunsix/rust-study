fn main () {
    
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    
    // let example_closure = |x| x;
    //error 처음 추론 했던 타입으로 고정됨.
    let n = example_closure(5);
}