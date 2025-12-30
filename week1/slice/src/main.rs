fn main(){
    let mut s = String::from("hello world");

    let word = first_word(&s); // word는 값 5를 받습니다

    // s.clear(); // 이 코드는 String을 비워서 ""으로 만듭니다

    println!("{word}");

}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]

}