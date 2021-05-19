//ownership

fn main() {
    let (adjective, name) = two_words();
    let name = format!("{} {}", adjective, name);
    print_out(name);
}

fn two_words() -> (String, String) {
    (format!("Fellow"), format!("Rustaceans"))
}

fn print_out(name: String) {
    println!("Original {:?}", name);
    //let devowelised_name = remove_vowels(name.clone());
    let (name, devowelised_name) = remove_vowels(name);
    println!("Devowelised {:?}", devowelised_name);
    println!("Removing vowels from {:?} yields {:?}", name, devowelised_name);
}

fn remove_vowels(name: String) -> (String, String) {
    let mut output = String::new();
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                continue;
            }
            _ => {
                output.push(c);
            }
        }
    }
    (name, output)
}