fn main() {
    let mut name = format!("Hello Rustaceans");
    
    {
        let r: &String = &name;
        //name.push('x');
        print_out(r);
        print_out(r);
        print_out_str(&r[1..]);
    }
    name.push('x');

    let mut sum: usize = 0;
    for word in name.split(' ') {
        sum += word.len();
        println!("{}", word);
    }
    println!("{}", sum);
}

fn print_out(name: &String) {
    println!("{}", name);
}

fn print_out_str(name: &str) {
    println!("{}", name);
}