// fn main() {
//     let a: [i32; _] = [1, 2, 3];
//     let mut v:Vec<i32> = Vec::new();
//     v.push(1);
//     v.push(2);
//     v.push(3); 
//     {
//         let v2 = vec![1,2,3];
//     }
//     // v2 scope is dropped after sometimes
// }


// fn main() {
//     let v: Vec<i32> = vec![1,2,3,4, 5];

//     let third = &v[2];

//     match v.get(2){
//         Some(third ) => println!("the third element is {}", third),
//         None => println!("There is no third element"), 
//     }
// }


// -----------------------------------------------
//                  Format                      
//-----------------------------------------------

// use std::fmt::format;

// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("World!");
//     let s3 = format!("{}{}", s1, s2);
// }


//--------------------------------------------------------
//                  HashMap                             
//--------------------------------------------------------

use std::{collections::HashMap, hash::Hash};

// fn main() {
//     let blue = String::from("Blue");
//     let yellow = String::from("Yellow");

//     let mut scores = HashMap::new();
//     scores.insert(blue, 10);
//     scores.insert(yellow, 50);

//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name);

//     for(key, value) in &scores {
//         println!("{}:{}", key, value);
//     }


// }

// -----------------------------------------------
// Updating hashvalue 
//------------------------------------------------

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 20);
//     scores.insert(String::from("Yellow"), 20);
//     scores.insert(String::from("Yellow"), 30);




// }

// --------------------------------------------------------
//              Updating hashmap based on old value
//---------------------------------------------------------

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}