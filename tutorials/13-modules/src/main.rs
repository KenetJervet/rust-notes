#![allow(dead_code)]

/* Rules of Module Filesystems:
 * 1. If a module named `foo` has no submodules, put it in `foo.rs`.
 * 2. If a module named `foo` has submodules, put it in file `foo/mod.rs`.
 * 3. The above rules apply recursively.
 */

mod defining_modules;

use defining_modules::shape;
use defining_modules::shape::area::CanCalcArea;  // Trait must be in scope to use

fn main() {
    let rect = shape::Rectangle { width: 5.0, height: 6.0 };
    println!("The area of rect is: {}", rect.area());
}
