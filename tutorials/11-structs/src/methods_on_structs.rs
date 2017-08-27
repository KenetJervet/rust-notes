struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        // Note that in Rust we don't need to distinguish between the `.` and `->`
        // operators of C++ due to a feature called:
        // "automatic referencing and dereferencing"
        return self.length * self.width;
    }

    fn can_hold(&self, other: &Self) -> bool {
        return self.length > other.length && self.width > other.width;
    }

    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size}
    }
}

pub fn methods_on_structs() {
    let rect = Rectangle { length: 3, width: 4};
    println!("The area of rect is: {}", rect.area());

    let rect2 = Rectangle { length: 2, width: 3};
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    let square = Rectangle::square(9);
    println!("The area of square is: {}", square.area());
}
