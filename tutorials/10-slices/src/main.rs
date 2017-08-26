#![allow(unused_variables)]

// String slices are references to a part of a string
// String literals are slices

fn main() {
    without_string_slices();
    with_string_slices();
    string_slices_as_parameters();
    other_slices();
}


/* Without string slices */

fn without_string_slices() {
    let mut s = String::from("Hello world!");
    let idx = first_word(&s);
    s.clear();
    // Now idx is useless
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()  // Note: length in `bytes`
}

/* With string slices */

fn with_string_slices() {
    let s = String::from("Hello world!");
    let slice = first_word_slice(&s);
    // s.clear();  // Err: slice is an immutable reference and thus
                   // borrowing s as mutable in String::clear() is impossible
    println!("{}", slice);
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    return &s[..]
}


fn string_slices_as_parameters() {
    let s = String::from("Hello world!");
    let first_word = first_word_better_version(&s);
    println!("First word: {}", first_word);
    let s2 = "Hello world";
    let first_word2 = first_word_better_version(&s2);
    println!("First word 2: {}", first_word2);
}

/* It's better for a function to accept &str than &String because it's more general */
fn first_word_better_version(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    return &s[..]
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];  // type: &[i32]
}
