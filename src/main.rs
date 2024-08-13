#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


impl Coin {
    fn value(&self) -> u8 {
        match self {
            Coin::Penny => {1},
            Coin::Nickel => 5,
            Coin::Dime => {
                println!("You have a dime!");
                10},
            Coin::Quarter(state) => {
                println!("Quarter's state is: {state:?}");
                25},
        }
    }   
}
fn main() {
    let coin : Coin = Coin::Quarter(UsState::Alabama);
    println!("Value of coin: {:?}", coin.value());
}
