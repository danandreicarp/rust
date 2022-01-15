// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

#[allow(unused)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(unused)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message received: {:?}", self);
    }
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

fn main() {
    ip_addr_as_enum_play();

    message_enum_play();

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dice_play();

    if_let_play_2(Coin::Penny);
}

// fn ipAddressKind_as_enum_play() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     route(four);
//     route(six);
// }

// fn route(ip_kind: IpAddrKind) {
//     println!("Routing {:?}", ip_kind);
// }

// fn ipAddr_as_struct_play() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

fn ip_addr_as_enum_play() {
    #[allow(unused)]
    let home = IpAddr::V4(127, 0, 0, 1);
    #[allow(unused)]
    let loopback = IpAddr::V6(String::from("::1"));
}

fn message_enum_play() {
    let quit_message = Message::Quit;
    quit_message.call();

    let move_message = Message::Move { x: 10, y: 20 };
    move_message.call();
    Message::Write(String::from("Some String")).call();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
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

fn dice_play() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other), // used when we need the value
        // _ => reroll(),               // used when we don't need the value
        _ => (), // used when nothing should happen in this case
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {}

fn reroll() {}

fn if_let_play() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // this is equivalent to the above
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn if_let_play_2(coin: Coin) {
    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("Number of coins: {}", count);
}
