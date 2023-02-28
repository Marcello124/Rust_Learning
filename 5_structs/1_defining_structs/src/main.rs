fn main() {

    let mut user = build_user(String::from("someusername123"), String::from("someemail@example.com"));

    user.email = String::from("anotheremail@example.com");

    println!("{}", user.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // println!("{}", black.0);

    let subject = AlwaysEqual;
}
// struct OtherUser {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
struct AlwaysEqual;
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email, // struct fields and parameters names are the same
        sign_in_count: 1,
    }
}