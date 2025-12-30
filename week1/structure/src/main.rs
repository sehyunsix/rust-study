struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        ..user1
    };
    println!("user2 email: {}", user2.email);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin point: ({}, {}, {})", origin.0, origin.1, origin.2);

}   

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}