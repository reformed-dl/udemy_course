/*Hash Maps - collection type that consists of key-value pairs
Keys must be unique, Values can be duplicated
Other languages -> dictionary, hash, associative array, map
Order is not important, but focuses on connections(mappings) think a restaurant menu. Dish Name is the Key, Price is the Value. Order is not imporant, but the connections are.
Stored on the heap, dynamic.

found within the standard collections library - use std::collections::Hashmap;

Hashmap::new(); -> New constructor function*/

use std::collections::HashMap;
fn main() {
   let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Steak"), 29.99);
    menu.insert(String::from("Tuna"), 29.99);
    menu.insert(String::from("Burger"), 13.99);
    println!("{:?}", menu);

    let mut city_capitals = HashMap::<&str, &str>::new(); //turbofish construction
    city_capitals.insert("Tallahassee", "Florida");
    city_capitals.insert("Albany", "New York");
    city_capitals.insert("Annaheim", "California");
    println!("{:?}", city_capitals);

    //Rust will also infer the Key and Value types once values are provided
}
