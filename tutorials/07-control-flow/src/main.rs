#![allow(unused_variables)]

fn if_expression() {
    // `if` expressions (NOT statements!)
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Since `if`s are expressions, we can do things like this:
    let z = if number > 5 { true } else { false };

    // As you can guess, each arm of the whole if expression
    // must return values of the same type
    println!("Z: {}", z);
}

fn loops() {
    // *** Forever: loop
    println!("*** loop ***");
    let mut number = 3;
    let z = loop {
        println!("again");
        number -= 1;
        if number == 0 {
            break;
        }
    };

    // *** While loop
    println!("*** while ***");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    // *** For loop
    println!("*** for ***");
    for element in [10, 20, 30].iter() {
        println!("{}!", element);
    }
    // Also sweet:
    for element in (1..4).rev() {
        println!("{}!", element);
    }
}

fn main() {
    if_expression();
    loops();
}
