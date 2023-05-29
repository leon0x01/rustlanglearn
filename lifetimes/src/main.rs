// ===============================================================//
//                          LifeTimes                            //
//==============================================================//
/* 
fn main() {

    let r: &i32;                //------------------ +---- a'
    {                           //-------------------|
        let x: i32 = 5;         // -----+ b'         |
        r = &x;                //-------|            | 
    }                          //-------+            |
    println!("r:{}", r);      //---------------------|
}                              //--------------------|


// life time of x is finished in above scope so that and r is pointing the reference of 
// x so R is known as Dangling repferences

// Dangling reference is a term used o describe a refrence that points to a memory 
// that has been deallocated or is no longer valid.
// It occurs when a reference outlives the data it refers to,leading to undefined behaviours when
// the refrence is accessed.

*/
/* 
fn main() {

    let x: i32 = 5;             //----------------------+--- 'b
    let r: &i32 = &x;           //------+--- 'a         |
    println!("r:{}", r);        //------|               |
}                               //----------------------+
*/


// fn main(){
//     let string1 = String::from("abcd");
//     let string2 = String::from("xyz");

//     let result = longest(string1.as_str(), string2.as_str());

//     println!("the longest String is {}", result);
// }

// // &i32         // a reference
// // &'a i32      // a reference with an explicit lifetime
// // &'a mut i32  // a mutable reference with an explicit lifetime

// fn longest<'a>(x: &'a &str, y: &'a &str) -> &'a &str{
//     if x.len() > y.len(){
//         x
//     }
//     else {
//         y 
//     }
// }

// above without generice life time compiler say missing lifetime specifier
// cause while returning str it it brrowed from x and y but compiler doesn't how to specify lifetime
// of x and y


//local variable declare in function should not pass as return of the funtion
// IT should be one of the parameter.


// Rules of Life time

//1. Each parameter that is a reference gets its own lifetime parameter

// 2. If there is exactly one input lifetime paramter, that lifetime is assigned to all output
// lifetime parameters;

// 3. If there are multiple input lifetime parameters, but one of them is &self and &mut
// self the lifetime of self is assigned to al output lifetime parameters.


// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// fn main() {
//     let novel = String::from("call me Hero. Some years ago...");
//     let first_sentence = novel.split('.').expect("cou")
// }

/*
  life time Annotation

  - Syntax starts with an ' followed by a small xase name which is generally 
    a single letter.
  - fn<'a>foo(param1: &'a type) -> &'a return_value
  - fn<'a>foo(param1: &'a mut type) -> &'a return_value
 */


// how do we calculate 'a ?

// 'a referes to the lifetime in which all the related references are valid, which is the
// overlap all of the references' lifetimes

// As a result, the smallest lifetime  of all the references gets assigned to'a 


// fn main(){
//     let i = 10;
//     println!("{}", i);
//     println!("{}", i);
// }
// #[derive(Debug)]

// struct Person{
//     name: String,
//     age: i32,
// }

// fn get_older_person<'a> (first_person: &'a Person, second_person: &'a Person) -> &'a Person{
//     if first_person.age > second_person.age{
//         first_person
//     }
//     else {
//         second_person
//     }
// }

// fn main() {
//     let p1 = Person {
//         name: String::from("ujjwal"),
//         age: 13,
//     };

//     let p2 = Person {
//         name: String::from("Asmit"),
//         age:20,
//     };

//     println!("{:?}", get_older_person(&p1, &p2));
// }


// #[derive(Debug)]

// struct Person{
//     name: String,
//     age: i32,
// }

// fn get_older_person<'a> (first_person: &'a Person, second_person: &'a Person) -> &'a Person{
//     if first_person.age > second_person.age{
//         first_person
//     }
//     else {
//         second_person
//     }
// }

// fn main() {
//     let older: &Person;
//     let p1 = Person {
//         name: String::from("ujjwal"),
//         age: 13,
//     };

//     {

//     let p2 = Person {
//         name: String::from("Asmit"),
//         age:20,
//     }; 
//     older = get_older_person(&p1, &p2);
// }
//     println!("{:?}", older);
// }

#[derive(Debug)]
struct Person{
    name: String,
    age: i32,
}

#[derive(Debug)]

struct Employee {
    person:  &Person,
}



fn main() {
      let employee: Employee;
      {
        let person = Person{
            name: String::from("ujjwal"),
            age: 22,
        };
        
        employee =  Employee{
            person: &person
        }; 
    println!("{:?}", employee); 
      }
   }



