struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user1 = User {
        email: String::from("nagendra@rust.com"),
        username: String::from("nagendra"),
        sign_in_count: 1,
        active: true
    };
    user1.email = String::from("someone@rust.com");
    println!("{}", user1.email);

    let user2 = give_user(format!("nagendra"), format!("nagendra@rust.com"));
    println!("{}", user2.email);

    let user3 = User {
        ..user1
    };
    println!("{}", user3.active);
}

fn give_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 2,
        active: true
    }
}