pub mod shape {
    pub struct Rectangle {
        pub width: f64,
        pub height: f64
    }

    pub struct Circle {
        pub radius: f64
    }

    pub mod area {
        pub trait CanCalcArea {
            fn area(&self) -> f64;
        }

        impl CanCalcArea for super::Rectangle {
            fn area(&self) -> f64 {
                self.width * self.height
            }
        }

        impl CanCalcArea for super::Circle {
            fn area(&self) -> f64 {
                3.14159 * self.radius * self.radius
            }
        }
    }
}
