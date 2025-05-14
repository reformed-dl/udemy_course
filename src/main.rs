/*HashMaps are stored on the Heap and follow the same ownership rules as other Heap Types.
The HashMap takes ownership of data assigned to it that do not implement the copy trait*/

use std::collections::HashMap;
fn main() {
   let drink = String::from("Latte");
   let milk = String::from("Oat Milk");
   let mut coffee = HashMap::new();
   coffee.insert(drink, milk);//ownership is transferred from the drink, milk variables to the HashMap
   println!("{:?}", coffee);
   println!("{}", coffee.len());//len() provides the number of Key-Value pairs contained within the HashMap (1)

   //There are options to retain ownership
   //We can pass in references to the Strings &String
   //Or we can hardcode the HashMap type with &str and then pass in references to the Strings due to the dereferencing feature
   let mut capitals: HashMap<&str, &str> = HashMap::new();
   let city = String::from("Tallahassee");
   let state = String::from("Florida");
   capitals.insert(&city, &state);
   capitals.insert("Albany", "New York");//this allows us the flexibility to pass in new string slices and not have type mismatches
   println!("{:?}", capitals);
   println!("{} {}", city, state);
}
