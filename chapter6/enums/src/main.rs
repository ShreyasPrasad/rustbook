enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrRevised {
    // You can put any kind of data inside an enum variant: strings, numeric types, or structs
    // We attach data to each variant of the enum directly, so there is no need for an extra struct
    V4(String), 
    V6(String), 
}

fn route(ip_kind: IpAddrKind) {} // takes ip addresses as an argument

fn definingenums(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let home = IpAddrRevised::V4(String::from("127.0.0.1")); // We automatically get the V4() constructor function defined as a result of defining the enum
    let loopback = IpAddrRevised::V6(String::from("::1"));
}


enum Message { // much more concise than defining individual structs
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message { // we can define methods on enums
    fn call(&self) {
        // method body would be defined here
    }
}

fn moreaboutenums(){
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn aboutoptions(){
    /*
        Option is an enum with 2 variants that expresses if a value is something or nothing.
        This is similar to null in other languages. Rust does not have null (it is a poor
        language construct that has caused countless errors in the past). Instead, 
        Option<T> is used to encode the concept of a value being presernt or absent.

        enum Option<T> {
            None,
            Some(T),
        }

        Variants are included in the prelude.
    */

    let some_number = Some(5); // The type of some_number is Option<i32>, inferred by Rust
    let some_string = Some("a string");

    // have to convert Option<T> to T before using T, requiring the developer to explicitly handle None
    if(some_string.is_some()){
        println!("{}", some_string.unwrap());
    }
    let absent_number: Option<i32> = None;
}

fn main() {
    definingenums();
    moreaboutenums();
    aboutoptions();
}
