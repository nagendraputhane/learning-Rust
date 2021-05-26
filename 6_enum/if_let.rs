fn main() {
    let some_value = Some(3u8);
    if let Some(3) = some_value {
        println!("three");
    }
}