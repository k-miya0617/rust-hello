use std::fmt;

struct Password(String);

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.chars().map(|_| '*').collect::<String>())
    }
}

struct Password2(String);
impl fmt::Display for Password2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A")
    }
}

struct User {
    email: String,
    name: String,
    signin_count: i32,
    is_active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "email: {},\nname: {},\n", self.email, self.name)
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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let a = String::from("123456789");
    println!("{}", a);

    let a = Password(String::from("123456789"));
    println!("{}", a);

    // let b = Password2(String::from("abc"));
    // println!("{}", b);

    let user = User {
        email: String::from("miya@gmail.com"),
        name: String::from("miya"),
        signin_count: 1,
        is_active: true,
    };

    println!("{}", user.name);
    println!("{}", user);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area is {}", rect1.area());
}
