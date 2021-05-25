#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

/*
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, //anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    route(four);

    /*
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };*/
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("Loopback -> {:?}", loopback);

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {
    println!("Home -> {:?}", ip_kind);
}