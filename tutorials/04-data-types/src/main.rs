#![allow(unused_variables)]

mod compound_types;
use compound_types::compound_types;

/*

Integer types:

|----------------------------|
| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| arch   | isize  | usize    |
|----------------------------|

Note: `arch` means architecture-dependent i.e. 32-bit for 32-bit arch and 64-bit for 64-bit arch

|-------------------------------|
| Number literals | Example     |
|-----------------|-------------|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |
|-------------------------------|


Floating-point types:

|-----------------|
| Length | Signed |
|--------|--------|
| 32-bit | f32    |
| 64-bit | f64    |
|-----------------|

*/

fn main() {
    // NOTE: Type annotation is required because `parse()` is polymorphic.
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("You guessed {}", guess+1);

    // Rust's floating numbers: f32 and f64 (default). They are roughly the same speed.
    // In 32-bit systems f64 is slower.
    let x = 2.0;  // f64 (the default)
    let y: f32 = 3.0;

    // Numeric operations:
    // +, -, *, /, %
    println!("{}", 3 / 2);  // Output: 1

    // Boolean type: bool (true / false)
    let t: bool = true;
    let f: bool = false;

    // Character type: char (unicode)
    let c: char = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    let guo = '国';

    // Compound types (compound_types.rs)
    compound_types();
}
