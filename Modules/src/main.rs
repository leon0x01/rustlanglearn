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

// mod file_3;

// fn main() {
//     file_3::allowance();
// }


use array_tool::vec::*;

fn main(){
    let vec1 = vec![1,1,3,5,6,7];
    let vec2 = vec![1,2,3];
    let intersection = vec1.intersect(vec2.clone());
    println!("The intersection  = {:?}", intersection);

    let union = vec1.union(vec2.clone());
    println!("The union = {:?}", union);

    println!("vec 1 three times displayed = {:?}", vec2.times(3));
}