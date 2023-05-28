### Traits

- Traits is a feature used for defining shared behaviour across different types.
- They are similar to interfaces in other programming languages.
- Traits define a set of methods that a type must implement in order to satisfy that trait.

To define a trait in Rust, you use the `trait` keyword followed by the name of the trait. Inside the trait block, you can declare associated types, constants and method  signatures. Here's an example:

```
    trait Printable {
        fn print(&self);
    }
```

In above example, we define a trait named **Printable** that has one method **print()**.

Types that implement this trait must provide an implementation for the `print()` method.

- To implement a trait for a specific type, you can the *impl* keyword. Here's an example of implementing the **Printable** trait for a struct named **person**:

```
    struct Person{
        name: String,
    }

    impl Printable for Person {
        fn print(&self){
            println!("Name: {}", self.name);
        }
    }
```
Here, we implement the **Printable** for the **print()** method and instance of Person can be used wherever **Printable** is expected.


- Traits allow you to define shared behavior across different typs without relying on inheritance. 
- It promote code reuse and enable generic programming in Rust.
- Traits help us to define default implementations for methods or to specify constraints on generic types.