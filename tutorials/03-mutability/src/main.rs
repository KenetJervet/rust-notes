#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_mut)]

fn main() {
    /* MUTABILITY */
    let immutable = 3;
    println!("The value of immutable is {}", immutable);
    // ERROR! Mutate an immutable variable
    // immutable = 4;

    let mut mutable = 3;
    println!("The value of mutable is {}", mutable);
    mutable = 4;  // OK!
    println!("The value of mutable is {}", mutable);

    /* CONSTANTS */
    // Constants can be declared at any scope, including global
    const MAX_POINTS: u32 = 100_000;

    /* SHADOWING */
    let shadowed_variable = 3;
    println!("The value of shadowed_variable is {}", shadowed_variable);
    let shadowed_variable = 4;  // Shadows the previous definition
    println!("The value of shadowed_variable is {}", shadowed_variable);

    let spaces = "    ";
    let spaces = spaces.len();  // OK!

    let mut spaces = "   ";
    // spaces = spaces.len();  // Mismatched types
}
