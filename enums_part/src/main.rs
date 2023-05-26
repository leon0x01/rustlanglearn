/* enum IpAddrKind{

    // hee we data to each variant here directly
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message{
    Quit,
    Move { x: i32, y: i32}, 
    Write(String),
    ChangeColor(i32, i32, i32), 
}
impl Message {
    fn call(&self) {
         // method body would be defineedher
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}



fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // datatypes is already defined in the enum directly so 
    // No need to use struct 
    let localhost = IpAddrKind::V4(127, 0, 0, 1); 

fn route(ip_kind: IpAddrkind){

}*/
/* 
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansa,
    California,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>{
            println!("Lucky Penny");
            1
        },
        Coin::Nickel  => 5,
        Coin::Dime => 10, 
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
}
*/

fn main() {
    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x:Option<i32>) -> Option<i32> {
    match x  {
        _ => None,
        Some(i) => Some(i+1),
    }
}