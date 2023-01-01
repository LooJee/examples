fn main() {
    let mut user1 = build_user(String::from("Jonathan"), String::from("jonathan@gmail.com"));

    user1.active = false;

    let user2 = User {
        username: String::from("Joshua"),
        ..user1
    };

    println!("user2.name : {}", user2.username)
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
