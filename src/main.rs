/*
Define a `make_money` function that accepts a mutable
String reference. The function should concatenate
the characters "$$$" to the end of the String.
Invoke the function in `main`.

Define a `trim_and_capitalize` function that accepts
a string slice. It should return a String with
all whitespace removed and all characters in uppercase.
Invoke the function in `main`.

Define an `elements` function that accepts a string
slice. It should split the string by all occurrences
of the `!` symbol and return a vector of the string
slices. Invoke the function in `main`.

Example:
elements("Gold!Silver!Platinum")
=> Vector of ["Gold", "Silver", "Platinum"]

Define a `get_identity` function. The function should
ask the user for their first and last name in TWO
steps (i.e., collect user input twice). Make sure
to communicate the instructions to the user.
For each Result enum you receive, call the `expect`
method and provide a custom error message (like
"Failed to collect first name"). Return a String
with the first and last names combined. Invoke
the `get_identity` function in `main`, and output the
returned String.

Example:
fn main() {
  let name = get_identity();
   println!("{name}"); // Bill Murray
}*/
use std::io;

fn make_money(text: &mut String) {
    text.push_str("$$$");
}
fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
    
}
fn elements(element: &str) -> Vec<&str> {
    element.split("!").collect()//if return type was not provided, you could use a turbofish operator .collect::<Vec<&str>>()
}
fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();
    let input = io::stdin();
    println!("What is your first name?");
    input.read_line(&mut first_name).expect("failed to collect first name");
    println!("What is your last name?");
    input.read_line(&mut last_name).expect("failed to collect last name");
    format!("{} {}", first_name.trim(), last_name.trim())
}
fn main() {
  let mut amount = String::from("1000");
  make_money(&mut amount);
  println!("{}", amount);

  let name = trim_and_capitalize("    Reformed and Salty   ");
  println!("{}", name);

  let element = elements("Volcano!Pyramid!Spaceship");
  println!("{:?}", element);

  let input_name = get_identity();
  println!("Nice to meet you {}", input_name);

  println!("Welcome to the world of shadowy coders {}", get_identity());
}
