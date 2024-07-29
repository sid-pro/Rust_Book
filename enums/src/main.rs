// Where structs give you a way of grouping together related fields and data, like a
// Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values.
//  For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddress {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Defining an enum with variants such as the ones is similar to defining different kinds of struct definitions,
//  except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type.
//   The following structs could hold the same data that the preceding enum variants hold:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method implementation goes here
        println!("Message");
    }
}
// But if we used the different structs, each of which has its own type, we couldn’t
//  as easily define a function to take any of these kinds of messages as we could with the Message enum

//option enums
// enum Option<T> {
//     None,
//     Some(T),
// }

// enum for match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
   Alaska,
   Hawai,
   Alabama
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Four {:?}", four);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home1 = IpAddress::V4(String::from("127.0.0.1"));
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    // if we want to define Option variable than we need to write Option explicitily
    let y: Option<i8> = Some(5);

    // it gives error because x and y are not of same type y can also be None
    // let sum = x + y;
    let sum = x + y.unwrap_or(0);
    println!("{}", sum);

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn route(ip_kind: IpAddrKind) {}

// Let’s break down the match in the value_in_cents function. First we list the match keyword followed by an expression,
// which in this case is the value coin. This seems very similar to a conditional expression used with if,
// but there’s a big difference: with if, the condition needs to evaluate to a Boolean value, but here it can be any type.
//  The type of coin in this example is the Coin enum that we defined on the first line.

// Next are the match arms. An arm has two parts: a pattern and some code. The first arm here has a pattern that is
//  the value Coin::Penny and then the => operator that separates the pattern and the code to run. The code in this case
//  is just the value 1. Each arm is separated from the next with a comma.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State quarter from {:?} has value 25", state);
            25
        },
    }
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x {
        Some(i) => Some(i+1),
        None => None,
    }
}
