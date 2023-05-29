// use std::collections::HashMap;
// fn main() {
//     let some_vec:Vec<i32> = vec![5,5,8,8,1,0,1,5,5,5,5];
//     let mut freq_vec:HashMap<i32, i32> = HashMap::new();

//     for i in &some_vec {
//         let freq = freq_vec.entry(*i).or_insert(0);
//         *freq += 1;
//     }
//     println!("{:?}", freq_vec);
// }
