// Ownership rules:
// 1. Each value is Rust as a variable that is called its `owner`.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

pub fn ownership() {
    the_string_type();
    move_();  // For stack+heap data, e.g. String
    clone();  // For stack+heap data, e.g. String
    copy();   // FOr stack-only data, e.g. i32
}

fn the_string_type() {
    let mut s = String::from("HELLO");
    s.push_str(", world");
    println!("{}", s);
}

fn move_() {
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}", s1);  // ERROR: use of moved value
}

fn clone() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);  // OK!
}

fn copy() {
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);
}
