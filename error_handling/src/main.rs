//------------------------------------------//
//            compiler suggestion in Panic Error//
//-------------------------------------------------//

// fn main(){
//     a();
// }

// fn a(){
//     b();
// }

// fn b(){
//     c(22);
// }
// fn c(num: i32){
//     if num == 22{
//         panic!("Don't pass in 22!");
//         // while comile the cod  it say  paniched at 'Don't pas in 22'
//         // To Trac  exact location how panic error is occuring
//         // It provide us some commad `RUST_BACKTRACE=1` cargo run" 
//         // It display flow or backtrace the code
//     }
// }


// ---------------------------------------------------------//
//             Recoverable Errors with Result 
//  - Using Result enum we can handle the Error and Recover it
//  - Result Enum is Simple to Option enum
//  - Option enum -> None Value or Some Value
//  - Result Enum -> Success or Error 
//-----------------------------------------------------------

// fn main() {
//     enum Result<T, E> {
//         Ok(T), // Ok store some generice value
//         Err(E), // E 
//     }
// }

// --------------------------------------------------------------//
//  Example of Result<T, E> Error handling
// ==============================================================//

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };
// }

// ===========================================================================//
//                          Error Propagation                                //
// ==========================================================================//

