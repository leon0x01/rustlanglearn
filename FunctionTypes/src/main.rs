// Basic Syntax and User

// fn main() {
//     let mut f = min;
//     println!("the minimum of two value is {}", f(2,3));
// }

// fn max(x: i32, y: i32) -> i32 {
//     if x > y {
//         x
//     }else {
//         y
//     }
// }
// fn min(x: i32, y: i32) -> i32 {
//     if x < y {
//         x
//     }else {
//         y
//     }
// }

/*//////////////////////////////////////////////////////
             Function types as paramter to functio
////////////////////////////////////////////////////////
/// */

// fn print_name(name: &str) {
//     print!("The name is {}", name);
// }


// fn prints_full_info(f: fn(&str), some_name: &str, age: i32){
//     f(some_name);
//     println!("and my age is {}", age);
// }

// fn main(){
//     let (myname, my_age) = (String::from("ujjwal"), 22);
//     prints_full_info(print_name, &myname, my_age);
// }

fn add_one(x: i32) -> i32{
    x+1
}

fn do_twice(f:fn(i32)-> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main(){
    println!("the value is {}", do_twice(add_one, 11));
}