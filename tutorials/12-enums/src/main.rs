#![allow(dead_code)]
#![allow(unused_variables)]



fn main() {
    defining_enums();
    the_match_operator();
    if_let();
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    ChangeColor(i32, i32, i32),
    Write(String)
}

fn defining_enums() {
    let msg = Message::Write(String::from("GG"));
    println!("{:?}", msg);

    // Option<T> is the same as `Maybe t` in Haskell
    // Some(a) <-> Just a
    // None    <-> Nothing
    let x: Option<i32> = Some(5);
    let y: i32 = 6;
    // println!("x + y = {}", x + y);  // type mismatch
}

/* The match operator */

fn default_to<T>(x: Option<T>, default: T) -> T {
    // Matches are exhaustive (unlike in Haskell)
    // There is also the _ placeholder
    match x {
        Some(a) => a,
        _ => default
    }
}

fn the_match_operator() {
    let option = Some(5);
    let option2: Option<i32> = None;

    let val = default_to(option, 10);
    let val2 = default_to(option2, 10);

    println!("val is {}", val);
    println!("val2 is {}", val2);
}

/* if let syntax */
fn default_to_with_if_let<T>(x: Option<T>, default: T) -> T {
    if let Some(val) = x {
        val
    } else {
        default
    }
}

fn if_let() {
    let option = Some(5);
    let option2: Option<i32> = None;

    let val = default_to_with_if_let(option, 10);
    let val2 = default_to_with_if_let(option2, 10);

    println!("val is {}", val);
    println!("val2 is {}", val2);
}
