// fn main() {
//   //--- Ownership rules -----
//   // 1. Each value in Rust has a variable that's called its owner.
//   // 2. There can only be onw owner at a time.
//   // 3. When the owner goes out of scope, the value with be dropperd.
//   {
//     // s is not valid here, it's not yet declared
//     let s: &str = "hello"; // s is valid from this point forwrad
//     // do stuffs with s
//   } // this scope is now over, and s is no longer valid
// }

// fn main() {
//   let x: i32 = 5;
//   let y: i32 = x; // Copy

//   let s1:String = String::from("hello");
//   let s2:String = s1.clone();

//   println!("{ }, world!", s1);

//   /* 
//     | name | value |          | index | value |
//     | ptr  |       |  ->      |   0   |  h    | 
//     | len  |   5   |          |   1   |  e    |
//     | capacity | 5 |          |   2   |  l    |
//                               |   3   |  l    |

//    */
// }

// fn main() {
//   // let s:String = String::from("hello");
//   // takes_ownership(s);
//   // println!("{}", s);

//   let x:i32 = 5;
//   makes_copy(x);
// }

// // fn takes_ownership(some_string: String) {
// //   println!("{}", some_string)
// // }

// fn makes_copy(some_integer: i32) {
//   println!("{}",some_integer);
// }


// fn main() {
//    let s1:String = gives_ownership();
//    let s2:String = String::from("hello");
//    let s3:String = takes_and_gives_back(s2);
//    println!("s1 = {} and s3 = {}", s1, s3);
// }

// fn gives_ownership() -> String {
//   let some_string: String = String::from("hello");
//   return some_string;
// }

// fn takes_and_gives_back(a_string: String) -> String {
//   return a_string
// }

// fn main() {
//     let s1:String = String::from("hello");
//     let len: usize = calculate_length(&s1);
//     println!("The length of '{}' i {}.", s1,len);
//     println!("{}", s1);
// }

// fn calculate_length(s: &String) -> usize {
//     let length: usize = s.len(); // lens returns thelength of a string
//     return length; 
// }


// fn main() {
//   let mut s: String = String::from("hello");
//   let r1: &mut String = &mut s;
//   //let r2: &mut String = &mut s;

//   println!("{}", r1);
// }

// fn main(){
//   // The rule of References
//   // 1. At any give time, you can have either one mutable reference or any number of immutable references.
//   // 2. References must always be valid.
//   let mut s: String = String::from("hello world");
//   let hello: &str = &s[..5];
//   let world: &str = &s[..];

//   let word:&str = first_word(&s);
//   s.clear();

// }

// fn first_word(s: &String) -> &str {
//   let bytes: &[u8] = s.as_bytes();
//   for (i, &item) in bytes.iter().enumerate() {
//     if item == b' ' {
//       return &s[0..i];
//     }
//   }
//   }

// fn main() { 
//    let s = String::from("hello");

//   let len = s.len();
//   let slice = &s[0..len];
//    println!("The first slice {}", slice);
//   let slice = &s[..];
//    println!("the Second slice {}", slice); 
   
// 

// fn first_word(s: &string) -> &str {
//   let bytes = s.as_bytes();

//   for(i, &item) in bytes.iter().enumerate() {
//     if item == b' ' {
//       return &s[0..i];
//     }
//   }

// }

fn main(){
  let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

let name = "hello";

let slice1 = &name[1..3];

assert_eq!(slice, &[2, 3]);

assert_eq!(slice1, "ell");

}





