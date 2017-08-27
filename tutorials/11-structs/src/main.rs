#![allow(dead_code)]

mod methods_on_structs;
use methods_on_structs::methods_on_structs;

fn main() {
    structs();
    tuple_structs();
    methods_on_structs();
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

/* Structs */

fn structs() {
    let user1 = create_user(String::from("foo@bar.com"), String::from("user1"));
    let user2 = create_user_from_other_user(user1);
    // println!("User1: {:?}", user1);  // However, now user1 is moved to the function above.
    println!("User2: {:?}", user2);
}

fn create_user(email: String, username: String) -> User {
    User {
        username,  // shorthand for `username: username`
        email,     // same as above
        sign_in_count: 1,
        active: true
    }
}

fn create_user_from_other_user(user: User) -> User {
    User {
        sign_in_count: 1,
        active: true,
        ..user
    }
}

/* Tuple structs */
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Unit();

fn tuple_structs() {
    let black = Color(0, 0, 0);
    println!("Black is: {:?}", black);

    let origin = Point(0, 0, 0);
    println!("Origin is: {:?}", origin);

    let unit = Unit();
    println!("Unit is: {:?}", unit);
}
