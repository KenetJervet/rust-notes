fn main() {
    println!("3 + 4.3 = {}", add(3, 4.3));
}

// Cannot omit any argument type or return value type
// unless you return void
fn add(a: i32, b: f64) -> f64 {
    (a as f64) + b
}
