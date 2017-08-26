// Recap: The Rules of References
// 1. At any time, you can have *either* but not both of:
//    a. One mutable reference
//    b. Any number of immutable references
// 2. References must always be valid.

fn main() {
    references();
    mutable_references();
    mutable_reference_scope_invalid();
    mutable_reference_scope_valid();
    immutable_references();
    dangling_references();
}

fn references() {
    let s = String::from("Hello");
    let l = calculate_length(&s);
    println!("The length of {} is {}", s, l);
}

fn mutable_references() {
    let s = String::from("Hello");
    // change(&mut s);  // Err: cannot borrow immutable as mutable

    let mut s = String::from("Hello");  // mutable variable
    change(&mut s);  // OK!
    println!("The new string is {}", s);

}

fn mutable_reference_scope_invalid() {
    let mut s = String::from("Hello");

    // There can only be one mutable reference to a particular data
    // in a particular scope:
    let r = &mut s;
    // let r2 = &mut s;  // cannot borrow `s` as mutable more than once at a time
    {
        // let r2 = &mut s;  // Same as above
    }
}

fn mutable_reference_scope_valid() {
    let mut s = String::from("Hello");
    {
        let r = &mut s;
        r.push_str("GG");
    }  // r goes out of scope
    let r2 = &mut s;  // And now it's okay to create a new mutable reference
}

fn immutable_references() {
    // It's okay to have multiple immutable references in the same scope
    let s = String::from("Hello");
    let r = &s;   // OK!
    let r2 = &s;  // OK!
}

fn dangling_references() {
    // let reference = dangle();
    //
    // fn dangle() -> &String {
    //     let s = String::from("Hello");
    //     &s                              // s is deallocated while the reference is returned
    // }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// The following function will cause compile error
// fn change(s: &String) {
//     s.push_str("GG");  // cannot borrow immutable borrowed content `*s` as mutable
// }

// The following function compiles fine
fn change(s: &mut String) {
    s.push_str(", world");
}
