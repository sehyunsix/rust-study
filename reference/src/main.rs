
fn main() {
    let mut s1 = String::from("hello");
    //scope 1
    let r1 =&mut s1;
    let len = calculate_length(r1);
    println!("The length of '{}' is {}.", s1, len);

    // //scope 2
    // let r2 =&mut s1;

    let s1= no_dangle();
    println!("The string {s1}");
    
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

// fn dangle()-> &String{
//     let s = String::from("hello");
//     &s
// }

fn no_dangle()->String{
    let s= String :: from("hello");
    s
}