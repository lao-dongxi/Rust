fn main() {
    let a=2;
    match a {
        1=>println!("{}",a+1),
        5=>println!("54"),
        other=>println!("{}",a+3),
    }
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) =some_u8_value  {
        println!("three");
    }


    
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
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("{}",count);
}