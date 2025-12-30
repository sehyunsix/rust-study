fn main() {

    {
        let  s1 = String::from("hello");
         
        let s2 = takes_ownership(s1);

        println!("{}, world!", s2);

        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }


}


fn takes_ownership(some_string : String)-> String{
    println!("{}",some_string);
    some_string
}