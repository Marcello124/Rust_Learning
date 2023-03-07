use std::string;

fn main() {
    let four = IpAddrVersion::V4;
    let six = IpAddrVersion::V6;

    // Inconvinient
    // let home = IpAddr {
    //     kind: IpAddrVersion::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrVersion::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // these are different types
    route(four);
    route(six);
}

enum IpAddrVersion {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_version: IpAddrVersion) {}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}