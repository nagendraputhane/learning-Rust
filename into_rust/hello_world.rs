fn main() {
    let lang = "Rust";
    println!("Hello, {:?}", lang);

    let name = format!("{}", "Nagendra");
    greet(name);
    //greet(name);
}

fn greet(name: String) {
    println!("Hello, {}", name);
}
