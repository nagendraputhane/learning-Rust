use std::io;

fn main() {
    let mut a = 0;
    let mut b = 1;
    println!("which Fibonacci number would you like?");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let mut n: u32 = n.trim().parse().expect("Not a number");
    println!("The number is");
    if n == 0 {
        println!("nada");
    }
    else if n == 1 {
        println!("{}", a);
    }
    else if n == 2 {
        println!("{}", b);
    }
    else {
        while n > 2 {
            let c = a + b;
            a = b;
            b = c;
            n = n - 1;
        }
    }
    println!("{}", b);
    
}