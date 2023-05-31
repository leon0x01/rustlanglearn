fn some_fn() {
    println!("Function of the file_1 crate");
}
mod maths{
    pub mod basic_maths {
        pub fn multiplication(num1: &i32, num2:&i32) -> i32{
            let result = num1 * num2;
            printing( &result);
            result
        }
    
    fn printing(num: &i32) {
        println!("The result is {}", num);
        crate::file_1::some_fn();
    }
}
}

// parent module can't see child module 
// Child module can see the private module private

pub fn rect_area(length: &i32, width: &i32) -> i32 {
    use maths::basic_maths::multiplication;
    multiplication(length, width)
}

// mod file_2;

// fn main() {
//     file_2::some_person();
// }