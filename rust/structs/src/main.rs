use std::println;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods: instructions that directly relate to the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Associative function: can be differentiated by not having the &self method
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 60,
        height: 70,
    };

    let rect3 = Rectangle::square(30);

    println!("rect: {:#?}", rect);
    println!("The area of the rectangle is {} square pixels", rect.area());
    println!("Rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("Rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("Print square {:#?}", rect3);
}
