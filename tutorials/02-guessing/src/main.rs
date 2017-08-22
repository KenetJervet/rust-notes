extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guessed_number: u32 = guess.trim().parse()
            .expect("Should be a number!");
        match guessed_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!")
        }
    }
    println!("The secret number is: {}", secret_number);
}
