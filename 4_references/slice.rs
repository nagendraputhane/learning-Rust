fn main() {
    let s = String::from("hello world");
    let word: &str = first_word(&s);
    //s.clear();
    println!("{}", &word[..]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}