#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_fit(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 15
    };
    let rect2 = Rectangle {
        width: 15,
        height: 14
    };
    println!("Area of rectangle 1 = {}", rect1.area());
    println!("Can rect2 fit in rect1? {}", rect1.can_fit(&rect2));
    
    println!("The square has dimensions {:#?}", Rectangle::square(3));
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}