### Closures

- Closures are anonymous functions that can capture values from their surrounding environment. 
- It is similar to lambdas or anonymous functions in other programming languages.
- Closoures provide us a convenient way to define functionality that can be stored in a vairable or passed as arguments to other functions.

few Examples -

```
    fn main(){
        let x = 5;
    

    // Closure taht captures 'x' from the surrounding environment
    let square = |num| num *x; 

    println!("{}", square(2)); // output: 10
    }

```

In above example using closure the **|num| num*x** syntax. The closure takes an argument **num** and mutliplies it by the captured variable **x** from the surrounding environment 

- The closure is assigned to the variable **square**

- when we call the **square** closure with the argument **2**. it multiplies **2** by the captured value of **x**

- Closures can caputre variable from the surrounding environment by reference or by value.
- By default, clsoures capture variables by reference, which means they borrow the values from the environment rather than taking ownership.
- This allows the closure to access the captured variables without taking ownership or modifying them.

### Example that demostrates closure variables by reference and modifying them using closure

```
fn main(){
    let mut x = 5;

    // Closure that captures 'x' by refernce and 
    // modify it inside closures
    let increment = || x +=1;

    increment(); // Increment x by 1
    println!("{}", x); // Output : 6
}
```
- The clousure **increment** captures **x** by reference(&x)
- When we call the increment closure, it increment the value of *x* by *1*. As a result, the value of *x* is modified to 6

- Closure can also Capture varaibles by value. this means they take ownership of the captured variables, allowding modfications that consume the values. To capture variables by value, we can use the **move** keyword before defining the closure

Example:

```
fn main() {
    let x = vec![1,2,3]

    // Closure that captures'x' by value and consumes it

    let consume = move || {
        for i in x {
            println!("{}", i);
        }
    };
    consume(); // Output: 1, 2, 3
    // println!("{:?}", x); // this line would cause a compilation error

}
```

- In above example, the closure **consume** captures the vector **x** by value using the **move** keyword. It then iterates over the elements of **x** and prints them. Since **x** was captured by value, the closure takes ownership of the vector and consumes it.


