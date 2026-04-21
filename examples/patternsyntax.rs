fn main () {


   let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        
        //새로운 y로 입력 답음. 기존의 scope이랑 다름.
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }


    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //matches
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    let data = vec![Some(1), None, Some(3), None];
    let count = data.iter()
      .filter(|x| matches!(x, Some(_)))
      .count();

    println!("{count}");

    let r: Result<i32, &str> = Ok(10);
    if matches!(r, Ok(n) if n >= 10) {
      println!("success and bigger value");
    }

}