// struct User {
//     username: String,
//     email: String, 
//     sign_in_count: u64,
//     active: bool,
// }
// fn main() {
//     let mut user1 = User {
//         email: String::from("hello@gmail.com"),
//         username: String::from("hello123"), 
//         active: true,
//         sign_in_count: 1
//     };
//     user1.username = String::from("Ram");
//     println!("The name of the person {}", user1.username);

//     let user2 = build_user(String::from("nepal@gmail.com"), String::from("shyam"));

//     let user3 = User{
//         email: String::from("ramro@gmail.com"),
//         username: String::from("username"),
//         ..user2
//     };

//     println!("{}", user3.active );
// }


// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         sign_in_count: 1,
//         active: true,
//         }
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectange { 
    fn area(&self) -> u32 {
        self.width * self.height;
    }
}

fn main(){
    let rect = Rectangle{
        width: 30,
        height: 50,
    };

    println!(" rect: {:#?}",rect);
    println!("The area of the reactangle is {} square pixels.",
    rect.area());
}

fn area(rectangle: &Rectangle) -> u32 { 
     rectangle.width * rectangle.height
    }
