pub fn main() {
    let string = format!("my friend");
    let r: &String = &string;
    greet_f(r);
    greet_f(&r[3..]);
}

fn greet(name: &String) {
    println!("Hello, {}!", name);
}

fn greet_f(name: &str) {
    println!("Hello, {}!", name);
}

// Goal #1: Convert `greet` to use borrowing, not ownership, so that
// this program executes without doing any cloning.
//
// Goal #2: Use a subslice so that it prints "Hello, friend" instead of
// "Hello, my friend".
