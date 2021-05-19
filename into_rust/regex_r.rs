//check if a string purely consists of whitespace

extern "C" fn fast_blank(s: &String) -> bool {
    for c in s.as_bytes() {
        println!("{}", c);
        if c == 32 {
            continue;
        }
        else {
            return false;
        }
    }
    return true;
}
fn main() {
    let s = String::from("Hello world");
    let b: bool = fast_blank(&s);
}