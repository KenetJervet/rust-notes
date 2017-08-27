pub fn strings() {
    string_concatenation();
    string_indexing();
    string_slices();
    iterating_over_strings();
}

fn string_concatenation() {
    let s1 = String::from("Hello");
    let s2 = String::from(", world!");

    // Various approaches to concatenating strings
    let mut s = s1.clone();
    s.push_str(", world!");

    let mut s = s1.clone();
    let mut s = String::from("Hello");

    let s = s1 + &s2;  // NOTE: s1 is moved
}

fn string_indexing() {
    // NOTE: Strings do not support indexing
    // See: https://doc.rust-lang.org/book/second-edition/ch08-02-strings.html
    // for details.
    let hello = "Здравствуйте";
    // let answer = &hello[0];  // Error
}

fn string_slices() {
    let hello = "Здравствуйте";
    println!("{}", &hello[..4]);  // Зд
    // println!("{}", &hello[..3]);  // Error: panic
    // let z = &hello[..1];  // Error: panic
}

fn iterating_over_strings() {
    let s = "नमस्ते";
    for c in s.chars() {
        println!("{}", c);
    }

    for b in s.bytes() {
        println!("{}", b);
    }
}
