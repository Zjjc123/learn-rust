struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("test@example.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("new_user@example.com");
    println!("user1 email: {}", user1.email);

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1 // remaining fields should get corresponding value from user1
    };

    println!("user1 email: {}", user1.email);
    // println!("user1 username: {}", user1.email); not valid because string is moved to user2
    println!("user1 email: {}", user2.email);

    // tuple struct (struct name)
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // unit-like structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
