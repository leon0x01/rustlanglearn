### Generics

- Generics is the feature that allows us to write code with multiple types. 
- they enable scope to define functions, structs, enums, and traits that can be parametrized to accept different types as arguments or use them as associated types.
- Generics provide us code reusability and abstraction, as you an write functions or data structures that operate on a wide ranges of types without sacrificing types safety. 


- To define Generic function , you can add angle brackets **<>** after the function name to specify one or more generic type parametes. Here's an example of a generic function that swaps the values of two variables:

```
    fn swap<T>(a: &mut T, b: &mut T){
        let temp = *a;
        *a = *b;
        *b = temp;
    }

```

In above example, **T** is a generic type of paramete that represents the type of the variables begin swapped. The function **swap** can be called with any type that supports mutable references.

When calling a generic funciton, Rust infers the concrete type based on the types of the arguments.

```
    fn main() {
        let mut a = 5;
        let mut b = 10;
        swap(&a, &b);
        println!("a: {}, b:{}", a, b);
    }
```