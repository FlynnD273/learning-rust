#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold (&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
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
        height: 65,
    };

    // println! is a special case and converts to borrowed references automatically
    println!(
        "The area of a {} rectangle is {} square units",
        rect1, rect1.area()
        );

    if rect3.can_hold(&rect1) {
        println!("A rectangle of dimensions {} can hold a rectangle of dimensions {}", rect3, rect1);
    }
}
