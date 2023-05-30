
// fn main() {
//     let x:i32 = 5;
//     let square = |num: i32| println!("The square of {} is {}", num, num*num);
//     let square= |num: i32| println!("The cube of {} is {}", num, num*num*num);
//     square(x);

//     let y: i32 = 15;
//     square(y);

//     // It is reusable just like function but anynonmous function
// }

fn main() {
    let division_status = |y:f32| {
        if y != 0.0{
            true
        }else{
            false
        }
    };
    division(5.0, 10.0, division_status);  
    division( 5.0, 0.0, division_status);
}

fn division<F: Fn(f32) -> bool>(x:f32, y:f32, f:F){
    if f(y)  == true {
        println!("The division result is {}", x/y);
    }
    else {
        println!("Division is not possible");
    }

}