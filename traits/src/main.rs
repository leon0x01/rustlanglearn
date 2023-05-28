/* pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary{
    fn summarize(&self) -> String;
}

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("hello World"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("Look down"),
        content: String:: from("Look above the sky"),
        location: String::from("kathmandu"),
    };

    println!("Tweet summary:{}", tweet.summarize());
    println!("Article summary:{}", article.summarize());
} */

/* struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String,
}

trait General_info{
    fn info(&self) -> (&str, u8, char); // just function signature not full implementation

    fn country_info(&self) -> &str;
}

impl General_info for Person {
    fn info(&self) -> (&str, u8, char){
        (&(self.name), (self.age), self.gender)
    }

    fn country_info(&self) -> &str{
        &self.citizenship
    }
}

impl General_info for Student {
    fn info(&self) -> (&str, u8, char) {
        ((&self.name_std), (self.age), self.sex)
    }
    
    fn country_info(&self) -> &str{
        &self.country
    }
}

fn main(){

    let Person1 = Person{
        name:String::from("daleon"),
        citizenship: String::from("Nepal"),
        age: 19,
        gender: 'M',
        salary: 40_000,
    };

    let student1 = Student{ 
        name_std: String::from("Ram"),
        age: 20, 
        sex: 'M',
        country: String::from("Nepal"),
    };

    println!("The basic info of the person {:?}", Person1.info()); 
    println!("the country info of the person {:?}", Person1.country_info());
}
*/

struct Circle {
    radius: f32,
}

struct Rectangle {
    length: f32,
    width: f32,
}

trait Calculate {
    fn area(&self) -> f32; 
    fn perimeter(&self) -> f32; 
}

impl Calculate for Circle {
    fn area(&self) -> f32 {
        3.14*(&self.radius)*(&self.radius)
    }

    fn perimeter(&self) -> f32 {
        2.0*3.14*(self.radius)
    }
}

fn main(){
    let a1 = Circle{
        radius: 32.34,
    };
    println!("the area of the Circle is{:?}", a1.area());
}