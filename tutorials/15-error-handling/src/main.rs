#![allow(unused_variables)]
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;


fn main() {
    unrecoverable();
    recoverable();
    propagating_errors();
}

fn unrecoverable() {
    // By default, panic! macro will do `unwinding`;
    // See https://doc.rust-lang.org/book/second-edition/ch09-01-unrecoverable-errors-with-panic.html
    // for details.

    // panic!("Fuck no!");

    // If you want to let the OS clean up stuff for you, set `panic = 'abort'` in
    // the appropriate [profile] sections in `Cargo.toml`
    // See https://doc.rust-lang.org/book/second-edition/ch09-01-unrecoverable-errors-with-panic.html
    // for details.

}

fn recoverable() {
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // A Rust version of `Either` in Haskell???

    let f = File::open("hello.txt");
    let f1 = match f {
        Ok(ref file) => file,
        Err(ref error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    // Handling different types of errors
    let f2 = match f {
        Ok(file) => file,
        // match guard
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

    // unwrap and expect

    let f = File::open("hello.txt").unwrap();  // will panic on Err(_)
    let f = File::open("hello.txt").expect("Fuck no!");

}

fn propagating_errors() {
    let r1 = read_file1();
    let r2 = read_file2();
}

fn read_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    match f {
        Ok(mut f) => {
            let mut s = String::new();
            let f = f.read_to_string(&mut s);
            match f {
                Ok(_) => Ok(s),
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e)
    }
}

fn read_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
