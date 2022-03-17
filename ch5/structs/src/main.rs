fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1 username: {}", user1.username);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user2 username: {}", user2.username);

    user1.username = String::from("someusername123");


    let mut user3 = build_user(String::from("bob@example.com"), String::from("bob"));
    println!("user3 username: {}", user3.username);

    user3.username = String::from("bobby boy");

    println!("user1 username: {}\nuser2 username: {}\nuser3 username: {}", user1.username, user2.username, user3.username);
}

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
        sign_in_count: 1
    }
}
