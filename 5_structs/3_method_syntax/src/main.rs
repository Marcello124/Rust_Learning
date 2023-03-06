use std::f32::consts::FRAC_2_SQRT_PI;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_fit(&self, rectangle: &Rectangle) -> bool {
        if self.width > rectangle.width && self.height > rectangle.height {
            true
        } else {false}
    }
}

// structs can have multiple impl blocks
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {}", 
        rect1.area());

    if rect1.width() {
        println!("Width of the rectangle is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_fit(&rect2));
    println!("Can rect2 hold rect3? {}", rect1.can_fit(&rect3));

    let sqaure1 = Rectangle::square(10);

    dbg!(&sqaure1);
    println!("{}", sqaure1.width);
}
