// Where structs give you a way of grouping together related fields and data,
// like a Rectangle with its width and height, enums give you a way of saying
// a value is one of a possible set of values
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

// It is worth noting that:
// the name of each enum variant that we define also becomes a function that constructs an instance of the enum
// There’s another advantage to using an enum rather than a struct:
// each variant can have different types and amounts of associated data
#[derive(Debug)]
enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// It is also possible to define methods on enums
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home {:?}", home);
    println!("loopback {:?}", loopback);

    // We could also tackle this problem storing the data in the enum
    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));

    println!("home {:?}", home);
    println!("loopback {:?}", loopback);

    // Storing different data types
    let home = IpAddrEnum2::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum2::V6(String::from("::1"));

    println!("home {:?}", home);
    println!("loopback {:?}", loopback);

    // Methods on enums
    let m = Message::Write(String::from("hello"));
    m.call();
}
