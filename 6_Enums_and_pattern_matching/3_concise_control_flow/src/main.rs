fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // no boilerplate code
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter is from {:?}", state),
        _ => count += 1,
    }

    // This code does the same things as the one above
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State Quarter is from {:?}", state);
    } else {
        count += 1;
    }
}

#[derive(Debug)]
enum UsState {
    Alaska,
}
enum Coin {
    Penny,
    Quarter(UsState)
}