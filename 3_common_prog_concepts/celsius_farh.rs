use std::io;

fn main() {
    println!("1. Fahrenheit to Celsius \n 2. Celsius to Fahrenhite");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("failed to read line");
    let option: i32 = option.trim().parse().expect("Not a number");

    let mut celsius = String::new();
    let mut fahrenhite = String::new();

    if option == 1 {
        println!("Enter the temperature in Fahrenhite");
        io::stdin()
            .read_line(&mut fahrenhite)
            .expect("failed to read line");
        let fahrenhite: i32 = fahrenhite.trim().parse().expect("not a number");
        let celsius = (fahrenhite - 32) * 5 / 9;
        println!("{}", celsius);
    } else {
        println!("Enter the temperature in Celsius");
        io::stdin()
            .read_line(&mut celsius)
            .expect("failed to read line");
        let celsius: f64 = celsius.trim().parse().expect("not a number");
        let fahrenhite: f64 = (celsius * 9.0 / 5.0) + 32.0;
        println!("{}", fahrenhite);
    }
}
