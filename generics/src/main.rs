// =============================================================================//
//                   Generics 
//              - Motivation (reducing code duplication
//              - Generics in function
// ============================================================================//
/* fn square(x : i32) -> i32 {
    x*x
}
fn squaref32(x : f32) -> f32{
    x*x
}

fn main(){
    println!("the square of the number is {}", square(5));
    println!("the square of the number is {}", squaref32(5.5));
}
*/


// fn square<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
//     x+x
// }

// fn main() {
//     println!("The square of the number is {}", square(5));
//     println!("The square of the number is {}", square(5.0));

//}


struct point<T, U>{
    x: T,
    y: U,
}

impl <T: std::fmt::Debug, U: std::fmt::Debug> point<T, U>{
    fn printing(&self){
        println!("The value of point are {:?}, {:?}", self.x, self.y);
    }
}

fn main() {
    let p1 = point{
        x: 1,
        y: 1, 
    };

    let p2 = point{
        x: 2.0,
        y: 3.0,
    };

    let p3 = point{
        x: 1, 
        y: 1.0,
    };
    p1.printing();
}