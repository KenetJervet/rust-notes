#![allow(dead_code)]

/* Rules of Module Filesystems:
 * 1. If a module named `foo` has no submodules, put it in `foo.rs`.
 * 2. If a module named `foo` has submodules, put it in file `foo/mod.rs`.
 * 3. The above rules apply recursively.
 */

mod defining_modules;  // declare an external module

use defining_modules::shape;  // import stuff
// it's also possible to `use mod1::mod2::*`
use defining_modules::shape::area::CanCalcArea;  // trait must be in scope to use

fn main() {
    let rect = shape::Rectangle { width: 5.0, height: 6.0 };
    println!("The area of rect is: {}", rect.area());
}
