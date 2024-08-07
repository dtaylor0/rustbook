enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    for coin in vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter] {
        println!("{}", value_in_cents(coin));
    }
}
