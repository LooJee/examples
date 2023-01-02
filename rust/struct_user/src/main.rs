fn main() {
    let mut user1 = build_user(String::from("Jonathan"), String::from("jonathan@gmail.com"));

    user1.active = false;
    println!("user1.email : {}", user1.email);

    let user2 = User {
        username: String::from("Joshua"),
        ..user1 // move user1.email to user2.email
    };

    println!(
        "user2.name : {}, user2.email: {}",
        user2.username, user2.email
    );

    println!("user2: {:?}", user2);

    // will failed here, user1.email has moved
    // println!("user1.email : {}", user1.email);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
