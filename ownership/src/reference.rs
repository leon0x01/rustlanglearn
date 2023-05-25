fn main() {
    let s1:String = String::from("hello");
    let len: usize = calculated_length(s1);
    println!("The length of '{}' i {}.", s1,len);
    println!("{}", s1);
}

fn calculate_length(s: &String) -> usize {
    let length: usize = s.len(); // lens returns thelength of a string
    return length; 
}