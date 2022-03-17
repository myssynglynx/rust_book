#[derive(Debug)]
enum UsState {
    Alabama,
    Arkansas,
    Arizona,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Guam,
    Hawaii,
    Iowa,
    Idaho,
    Illinois,
    Indiana,
    Kansas,
    Kentucky,
    Louisiana,
    Massachusetts,
    Maryland,
    Maine,
    Michigan,
    Minnesota,
    Missouri,
    Mississippi,
    Montana,
    NorthCarolina,
    NorthDakota,
    Nebraska,
    NewHampshire,
    NewJersey,
    NewMexico,
    Nevada,
    NewYork,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Virginia,
    Vermont,
    Washington,
    Wisconsin,
    WestVirginia,
    Wyoming
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Coin: {:?}", value_in_cents(Coin::Penny));
    println!("Coin: {:?}", value_in_cents(Coin::Nickel));
    println!("Coin: {:?}", value_in_cents(Coin::Dime));
    println!("Coin: {:?}", value_in_cents(Coin::Quarter(UsState::California)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
