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

   //Accessing a Value from a Key
   let state = capitals["Tallahassee"];//enter the Key within the [] and the Value is returned. This will work as long as a existing Key is entered
   println!("{}", state);
   //If the Key does not exist, we will get a panic at runtime
   //get() - we wil return a Option Enum, the Sum Variant Associated data will be a reference is the Key exists to avoid owenership being transferred
   let states = capitals.get("Tallahassee");
   println!("{:?}", states);//Some(Florida)
   let error = capitals.get("Milwaukee");
   println!("{:?}", error);//because we entered a Key that doesn't exist, result is None

   //copied() makes a copy of the type and not a reference to the type
   let east_city = capitals.get("Albany").copied().unwrap_or("This Key Does Not Exist");
   println!("{}", east_city);

   let wrong_city = capitals.get("Tempe").copied().unwrap_or("This Key Does Not Exist");
   println!("{}", wrong_city);


}
