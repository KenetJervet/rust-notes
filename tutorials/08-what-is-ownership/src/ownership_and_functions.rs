pub fn ownership_and_functions() {
    let s = String::from("Hello");
    take_ownership(s);     // s's value moved into the function
    // println!("{}", s);  // and is no longer valid
    let s = 5;
    make_copy(s);          // x is copied into the function
    println!("{}", s);     // and is still valid

    // Return values and scope
    let s1 = give_ownership();
    let s2 = String::from("Hello");
    let s3 = take_and_give_back(s2);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn make_copy(s: i32) {
    println!("{}", s);
}

fn give_ownership() -> String {
    String::from("Hello")
}

fn take_and_give_back(s: String) -> String {
    println!("Take and give back {}", s);
    s
}
