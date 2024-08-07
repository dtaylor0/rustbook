use std::fmt;

#[derive(Debug, Default)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User: [name={}, email={}, active={}, sign in count={}]",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}

fn main() {
    // Users
    let mut user = User {
        active: true,
        username: String::from("John Doe"),
        email: String::from("john.doe@gmail.com"),
        sign_in_count: 129,
    };

    user.username = String::from("Jon Doe");
    println!("{:#?}", user);

    let mut user2 = build_user(
        String::from("Harold Haroldson"),
        String::from("hh@gmail.com"),
    );
    user2.sign_in_count += 1;
    println!("{user2}");

    // Rectangles
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let num = 165;
    let num = num.to_string();
    println!("{num}");
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
