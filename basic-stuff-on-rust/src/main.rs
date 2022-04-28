#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    utype: UserType,
}

enum UserType {
    Admin,
    User,
}

fn main() {
    let mut user1 = User {
        name: String::from("John"),
        age: 30,
        utype: UserType::Admin,
    };

    user1 = User {
        name: dbg!(String::from("Willy")),
        ..user1
    };

    if user1.utype == UserType::Admin {
        println!("He is an admin!");
    }

    println!("{:#?}", user1);
}
