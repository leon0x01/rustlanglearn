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

// fn main() {
//     let five = Some(5);
//     let six  = plus_one(five);
//     let none = plus_one(None);
// }

// fn plus_one(x:Option<i32>) -> Option<i32> {
//     match x  {
//         _ => None,
//         Some(i) => Some(i+1),
//     }
// }
/* enum Conveyance {
    Car = 15,
    Train = 20, 
    Air = 30,
}

impl Conveyance {
    fn travel_allowance(&self, miles:i32) -> f32 {
       let allowance =   match self {
            Conveyance::Car => miles as f32 * 14.0 * 2.0,
            Conveyance::Train => miles as f32 * 18.0 * 2.0,
            Conveyance::Air => miles as f32 * 34.0 * 2.0,

        };
        allowance
    }
}

fn main() {
    let participant_1 = Conveyance::Car;
    // println!("The value of the option is {}", participant_1 as i32);
    
    println!("The allowance of the participant 1 is {:?}", participant_1.travel_allowance(10));
}

*/
// #[derive(Debug)]
// enum Value{
//     Integer(i32),
//     Float(f32),
// }

// fn main() {
//     let some_value = vec![Value::Integer(32), Value::Float(1.0)];

//     println!("the value is {:?}", some_value[0]);


//     for i in some_value.iter(){
//         match i {
//             Value::Integer(num) => println!("The value is an interger type {}", num),
//             Value::Float(num) => println!("The value is an float type {}", num ),
//         }
//     }
// }

//  =====================================================


fn main(){
    let  disease: Option<String> = None;
    //disease = Some(String::from("Diabetes"));

    match disease {
        Some(disease_name) => println!("The disease is {}", disease_name),
        None => println!("You do not have disease"),    
    }
}