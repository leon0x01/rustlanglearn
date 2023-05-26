*enum* is a keyword used to define an enumeration type, also known as an "enum". An enum is a custome data type that represents a set of named values, where each values is called a variant.
- Enums are powerful for defining a type that can take on a limited number of distinct values.

- Enums allows enumerate the list of variance.
- Enums datatype can directly attach using simple bracket `()`
- In enums, you can put any kind of data inside an enum variant: strings, numeric types, or structs
- There is one more similarity between enums and structs: just as we're able to define methods on structs using impl, we're also able to define methods on enums. Here's a method name call that we could define on our Message enum:

```
     impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
```

### Option Enums

As many language has NULL values and null values represent useful concept it could be either exist or null. 
- The problem with null values is that the type system can't guarantee that if you use a value it's not null. But in rust there's are no null values but instead we have the option enum

```
enum Option<T> {
    None,
    Some(T),
}
```

- Option<T> enum is so useful becuase you don't need to bring it into scope explicitly.
- Its variants are also Included in the prelude:: you can use Some and None directly without the Option:: prefix