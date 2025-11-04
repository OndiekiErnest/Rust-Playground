#[derive(Debug)] // so we can inspect the state
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    // If the pattern matches,
    // it will bind the value from the pattern in the outer scope.
    // If the pattern does not match, the program will flow into the else arm,
    // and return from the function entirely.
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);

    value_in_cents(&coin);

    if let Some(desc) = describe_state_quarter(&coin) {
        println!("{}", desc);
    }
}
