/* Strings - a piece of text, or a sequence of characters 
"&str" reference to a string, called a string reference.
"String" dynamic, stored on the heap
When obtaining string slices, must provide a range example: let pirate = "Bloodhound"; let first letter = &pirate[0..1];*/


fn main() {
//Concatenation Example
let mut full_name = String::from("Sylvester");
let last_name = "Stallone";
full_name.push(' ');
full_name.push_str(last_name);
println!("{}", full_name);

//Concatenation using the add method
let first_name = String::from("Sylvester ");
let last_name = String::from("Stallone");

let full_name = first_name + &last_name;//must use & because + invokes a method on first_name and last_name becomes the argument. Ownership transfers from first_name to full_name
println!("{}", full_name);

//behind the scenes, this is what is being ran
//fn add(mut self, other: &str) -> String {
//self.push_str(other);
//self
}
