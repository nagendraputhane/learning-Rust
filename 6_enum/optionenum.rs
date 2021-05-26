fn main() {
    let absent_number: Option<i32> = None; //currently not known, but will be known with type as Option<i32>
    println!("{:?}", absent_number);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(absent_number);
    println!("six => {:?}, none => {:?}", six, none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}