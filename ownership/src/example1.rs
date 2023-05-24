fn main() {
    let string = "hello world";
    println!("{:p}", string); 
    foo(string);
}

fn foo(string: &str) {
    println!("{p}", string);
}