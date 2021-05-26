#[derive(Debug)]
enum UStates {
    Alabama,
    Alaska,
    Arkansas,
    California,
    Utah,
    NorthCarolina,
    SouthCarolina,
    NorthDakota,
    SouthDakota,
    NewMexico,
    Texas,
    Arizona,
    Nevada,
    NewYork,
    NewJersey,
    Maine,
    RoadeIsland,
    Delaware,
    Washington,
    Ohio,
    Pennsylvania,
    Kansas,
    Kentucky,
    Connecticut,
    Wisconsin,
    Michigan,
    Florida,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UStates),
}

fn value_in_cents(coins: Coin) -> u8 {
    match coins {
        Coin::Penny => {
            println!("lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    println!("the value in cents = {}", value_in_cents(Coin::Quarter(UStates::Florida)));
}