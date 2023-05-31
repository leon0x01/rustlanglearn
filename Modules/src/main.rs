// mod file_1;

// fn main() {
//     let rec1 = Rectangle{
//         length: 5,
//         width: 10,
//     };
//     file_1::rect_area(&rec1.length, &rec1.width);
// }

// struct Rectangle{
//     length: i32,
//     width: i32,
// }

// mod file_2;
// fn main() {
//     file_2::some_person();
// }

mod file_3;

fn main() {
    file_3::allowance();
}