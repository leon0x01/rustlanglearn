// ---------------------------------------------------------------------------------------
//              Smart pointers
//               - Box smart contracts stored pointer
//  


// fn main(){
//     let single_value = Box::new(0.625);
//     let x = 0.625;
//     println!("Are these values being equal {}", x == *single_value);

//     let mut stack_var = 4;
//     let stack_ref = &stack_var;
//     let heap_var = Box::new(stack_var);

//     stack_var = 5;
//     println!("The value of stack_var = {} and the heap_var = {}",stack_var, heap_var);

//     let point = Box::new((100, 125));
//     println!("{} {}", 100 == point.0, point.1);

//     let x = point;
//     println!("the value is {:?}", x);
// }

// enum list {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
//     let list = Cons(1, Box::new(Cons(1, Box::new(2, Box::new(Cons(3, Nil)))));
// }



// fn main() {
//     let a = 50;
//     let b = Box::new(a);
//     println!("{}", 50==a);
//     println!("{}", 50 == *b);

//     println!("{}", a == b);
// }

// use std::ops::Deref;
// // Deref
// struct MyBox<T>(T);

// impl<T> MyBox<T>{
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// impl<T> Deref for MyBox<T>{
//    type Target = T;
//    fn deref(&self) -> &T{
//         &self.0
//    } 
// }
// fn main() {
//     let x = 5;
//     let y = Box::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
//}


struct MySmartPointer{
    value: i32
}

use std::ops::Deref;

impl Deref for MySmartPointer {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.value
    }
}

impl MySmartPointer{
    fn new(x: i32) -> MySmartPointer{
        MysmartPointer {
            value: x
        }
    }
}


fn main(){
    let a = 50;
    let b = box::new(a);
    println!("{}", 50 == a);
    println!("{}", 50 == *b);
   // println!("{}", a == b);
}