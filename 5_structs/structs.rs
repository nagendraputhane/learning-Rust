#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 15
    };
    println!("The dimensions are {:#?}", rect1);
    println!("{}", area(rect1));
}

fn area(rect: Rectangle) -> u32 {
    rect.width * rect.height
}