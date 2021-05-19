// Goals:
// - change to greet you by name
// - introduce a variable `name` and insert this into the format string
// - try `println!("{}", name)` vs `println!("{:?}", name)` and see what the difference is

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
