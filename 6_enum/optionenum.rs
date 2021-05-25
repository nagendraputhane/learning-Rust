fn main() {
    let some_number = Some(5);
    let some_string = Some(" a string");

    let absent_number: Option<i32> = None; //currently not known, but will be known with type as Option<i32>
    println!("{:?}", some_number);
}