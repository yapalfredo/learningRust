enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let localhost = IpAddrKind::V4(String::from("127.0.0.1"));

    // let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // let c1 = Coin::Penny;

    // println!("{:?}", c1);

    // let q1 = Coin::Quarter(UsState::California);

    // println!("{:?}", q1)

    // value_in_cents(Coin::Quarter(UsState::California));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None,
        Some(i) => Some(i + 1),
        _ => None,
    }

    //other way of writing this
    // if let Some(3) = some_value {
    // println!("three");
    //}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    //....
}

#[derive(Debug)]
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
            println!("This is {:?}!", state);
            25
        }
    }
}
