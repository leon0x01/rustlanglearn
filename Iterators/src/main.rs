/* ////////////////////////////////////////////////
     - Basics of Iterators and its Syntax
*/


// fn main(){
//     let x  = vec![1,2,3,4,5,6,7];
//     let mut iter = x.iter();

//     println!("The iterator: {:?}", iter);
//     println!("{:?}", iter.next()); // return option enum
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
//     println!("{:?}", iter.next());
// }

//===============================================
//  soem useful functions for iterators 
// ==============================================

fn main() {
let a = vec![0,1,2,3,4, 5, 6];
     let mut check =  a.iter().any(|&x| x > 0);
     println!("The value of the any function is {}", check);

     let check = a.iter().all(|&x| x>= 0);
     println!("the value of vec is o or not {}", check);

     let check = a.iter().find(|&&x| x > 0);
     println!("the value of the function  is {}", check.unwrap());

     let check = a.iter().position(|&x| x>4);
     println!("The value of the function position is {}", check.unwrap());

     let check = a.iter().rposition(|&x| x > 4);

     println!("The value of the rpoistion is {}", check.unwrap());
//===========================================================
//             Common and Statistic function 
// ==========================================================

     let check = a.iter().max();
     println!("the max value in the function is {}", check.unwrap());

     let check = a.iter().min();
     println!("the max value in the function is {}", check.unwrap());

     let mut check = a.iter().rev();
     println!("The reverse value is {:?}", check.next());
}



