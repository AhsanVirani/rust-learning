struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let u1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("Sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2: User = set_email(u1);

    println!("{} {} {} {}", u2.email, u2.username, u2.active, u2.sign_in_count);

    println!("Success");    
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}