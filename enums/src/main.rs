#[derive(Debug)]
enum UsState {
    Arizona,
    California,
    Texas,
    Florida,
    Ohio
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: &Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(_) => 25
    }
}

fn state_of_quarter(coin : &Coin) -> Option<&UsState> {
    match coin {
        Coin::Quarter(state) => Some(state),
        _ => None
    }
}

fn main() {
    let purse : [Coin;6] = [
        Coin::Penny,
        Coin::Dime,
        Coin::Quarter(UsState::Arizona),
        Coin::Nickel,
        Coin::Quarter(UsState::California),
        Coin::Quarter(UsState::Ohio)
    ];

    for coin in purse {
        let value = value_in_cents(&coin);
        if let Some(state) = state_of_quarter(&coin) {
            println!("State of coin : {:?}",state);
        };
        println!("Value of coin : {}",value);
    }

}
