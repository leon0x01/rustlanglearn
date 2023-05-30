
// fn main() {
//     let x:i32 = 5;
//     let square = |num: i32| println!("The square of {} is {}", num, num*num);
//     let square= |num: i32| println!("The cube of {} is {}", num, num*num*num);
//     square(x);

//     let y: i32 = 15;
//     square(y);

//     // It is reusable just like function but anynonmous function
// }

// fn main() {
//     let division_status = |y:f32| {
//         if y != 0.0{
//             true
//         }else{
//             false
//         }
//     };
//     division(5.0, 10.0, division_status);  
//     division( 5.0, 0.0, division_status);
// }

// fn division<F: Fn(f32) -> bool>(x:f32, y:f32, f:F){
//     if f(y)  == true {
//         println!("The division result is {}", x/y);
//     }
//     else {
//         println!("Division is not possible");
//     }

// }


// three types of declaration of closure
/* fn main() {
    let some_closure_1 = || -> u32 {x+1};
    let some_closure_2 = |x| {x+1};
    let some_closure_3 = |x| x+1;
}
*/

// fn main() {
//     let mut vec1 = vec![1,2,3];
//     let some_closure = || {
//         println!("{:?}", &vec1); // infred to be used as immutable reference
//     };

//     println!("Vec 1: {:?}", vec1);
//     some_closure();
//     vec1[1] = 15;
// }

// fn main() {
//     let mut x = vec![1, 2, 3, 4, 5];
//     let mut some_closure = || {
//         x.push(6);
//         println!("Vector is {:?}", x);
//     };
//    // println!("Vector is {:?}", x);
//     some_closure();
//     println!("Vector is {:?}", x);
// }

fn main() {
    let mut x = vec![1,2,3];
    let mut some_closure = || {
        let y =x;
        println!("value : {:?}", y);
    };
    some_closure();
   // println!("The value of X :{:?}", x);
}