## HashMap

- It is a collection type that stores key-value pairs in an unordered manner. It provides fast lookup and insertion times making it efficient for mapping values to unique keys.
- It provider efficient key-value pair lookups and insertions.
- They are widey used for tasks such as caching, indexing and counting occurences of element.
- To use a Hashmap, we need to import the **HashMap** type from **std::collections** module. Here's an Example of how to create HashMap and use of Hashmap in Rust

```
use std::collections::HashMap;

fn main(){
    // create a new empty HashMap
    let mut map: HashMap<String, i32> = HashMap::new();

    // Insert key-value pairs into the HashMap
    map.insert(String::from("apple"), 5);
    map.insert(String::from("banana"), 2);
    map.insert(String::from("orange"), 8);


    //Access values by key 

    if let Some(quantity) = map.get("banana") {
        println!("Quantity of banana: {}", quantity);
    }

    // Update a value

    map.insert(String::from("apple"), 10);

    // Remove a key-value pair
    map.remove("orange")


    // Iterate over key-value pairs

    for(key,value) in &map {
        println!("{}: {}", key, value);
    }

}
```