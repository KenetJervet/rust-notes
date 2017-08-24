#![allow(unused_variables)]

mod scope;
use scope::scope;

mod ownership;
mod ownership_and_functions;

use ownership::ownership;
use ownership_and_functions::ownership_and_functions;

fn main() {
    scope();
    ownership();
    ownership_and_functions();
}
