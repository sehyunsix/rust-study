fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; 

    println!("===numericals===");
    println!("sum: {sum}");
    println!("difference: {difference}");
    println!("product: {product}");
    println!("quotient: {quotient}");
    println!("truncated: {truncated}");

    let c ='z';
    let z : char  = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("===characters===");
    println!("c: {c}");
    println!("z: {z}");
    println!("heart_eyed_cat: {heart_eyed_cat}");

    let t = true;
    let f: bool = false;
    println!("===booleans===");
    println!("t: {t}");
    println!("f: {f}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("===tuples===");
    println!("five_hundred: {five_hundred}");
    println!("six_point_four: {six_point_four}");
    println!("one: {one}");

    let a = [1, 2, 3, 4, 5];
    let b= [3;5];
    println!("===arrays===");
    println!("a: {:?}", a);
    println!("b: {:?}", b);

}