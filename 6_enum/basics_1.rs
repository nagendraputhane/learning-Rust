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
}

fn route(ip_kind: IpAddrKind) {
    println!("Home -> {:?}", ip_kind);
}