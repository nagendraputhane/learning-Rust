fn main() {
    let mut name = format!("Fellow Rustaceans");
    let r = &mut name;
    helper(r);
    //println!("{}", name);
    
    //let r2 = &mut name;
    helper(r);
    println!("{}", name);
}
fn helper(name: &mut String) {
    name.push_str("sss");
    println!("{}", name);
}