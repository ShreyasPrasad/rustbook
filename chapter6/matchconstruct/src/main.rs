#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime {
        big: String,
        small: String
    },
    Quarter(UsState), // quarters have state variants
}

fn value_in_cents(coin: Coin) -> u8 {
    /*
        match lets you compare a value to a series of patterns and execute against the pattern that matches
        the compiler ensures that the match covers all possible variants (cases) in the enum
        the difference between if statement and match is that match doesn't need to have boolean patterns (here they are types)
    */

    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }, // This is a match arm that consists of a pattern and then some code to run upon a match, separated using =>
        Coin::Nickel => 5,
        Coin::Dime{ big, small } => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn addOne(value: Option<i32>) -> Option<i32>{
    match value {
        Some(num) => Some(num+1),
        None => None // we need this case or our code won't compile (match forces us to handle all variants); matches are exhaustive
    }
}

fn dicegame(){
    let val = 10;
    match val { // match arms are processed in the order they appear
        3 => println!("you got a 3!"),
        9 => println!("you got a 9!"),
        other => println!("you got something that isn't 3 or 9!") // other is a catch all variable
    }
    // the catch all lets us add logic for cases we care about, and do nothing for all other cases
}

fn main() {
    println!("Hello, world!");
    let change = Coin::Quarter(UsState::Alabama);
}
