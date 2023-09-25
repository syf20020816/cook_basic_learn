use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct User {
    username: String,
    age: u8,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "user => name : {} , age : {}", self.username, self.age)
    }
}

fn main() {
    let user = User { username: "Matt".to_string(), age: 16 };

    println!("{}",user);
}
