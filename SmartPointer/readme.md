### Smart Pointers 

A smart pointer is a type that provides additional functionality and safety guarantees beyond what is offered by a regular raw pointer. Smart pointers are a key feature of Rust's ownership system and are used to manage memory and ensure memory safety.

**Box<T>**

- The **Box<T> type is used when you need to allocated memory on heap and store a value in it.
- It provides ownership of the allocated memory, and the memory is automatically deallocated when the **Box<T> goes out of scope.

```
fn main() {
    let x = Box::new(5); // Allocating memory on the heap
    println!("Value: {}", *x); // Dereferencing the box to access teh value
    
} // x goes out of scope, and the memory is deallocated 
```

- In above example, we created a **Box** that holds the values **5** on the heap. We can access the value by dereferencing the **Box** using *x. When the **Box** goes out of scope at the end of the **main** function, the memor is automatically deallocated.


### Deref 

- The **Deref** trait is used to enable the automatic derefrencing of smart pointers
- It allows you to customized the behaviour of the `*` operator
- It allows you to treats pointer like regular references.
- It enables automatic dereferencing, which means you can use the * operator(dereference operator) on a smart pointer to access the value it point to.

For example we have a smart pointer type called **MyBox<T>** that wraps a value of type *T*. By implementing the **Deref** trait for **MyBox<T>**, you can customize how the **`*`**  operator behaves when used with **MyBox<T>**.

```
use std::ops::Deref;

struct MyBox<T>(T);


impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &self::Target {
        &self.0
    }
}


fn main() {
    let x = 5;
    let my_box = MyBox(x);

    println!("Value: {}", *my_box);
}
```
in above Example, we define a type **MyBox<T>** that holds a value of type **T**. We implement the **Deref** trait for **MyBox<T>** b defining the associated type **Target** as **T**.This specifies that the target of the dereference operation should be the inner value of the **MyBox**.

- In the implemenation of **Deref::deref()**, we return a reference to the inner value by using the **&** operator and **self.0**.
- That means when we use the `*` operator on **my_box**, it will invoke the **Deref** trait's deref() method giving us ta reference to the inner value.

- when we write `*my_box`, the **Deref** trait implementation is called, and it effectively becomes `*(my_box.deref())`, allowing us to access the inner value of x.
