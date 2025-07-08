/* Trait Bounds to Conditionally Implement Methods
    *Trait bounds are when we attach one or more Traits to a Generic. It enforces constraints on what that possible Type could be. Any Type must implement the 
    given Traits. It introduces limitations/boundaries.
*/

use std::collections::HashMap;
use std::fmt::Display; // must add this use import to add the trait bound for Display in the impl block below

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32) -> ();
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

// Expanding Hotel Struct to accept a Generic Type T, once we make this change, all of the existing impl blocks will show an error as they expect a defined type.
#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservation: HashMap<String, u32>,
}
// In order to have this code work, we have to use a special syntax that lets Rust know that we want these methods to work on any generic T
// We will also add the trait bound here, so that Rust knows whatever type T is, it must implement the Display Trait
// We will split up these impl blocks so we have more flexibility on the different types we can use and not limit our constructor function
impl<T> Hotel<T> {
    fn new(name: T) -> Self {// we have to change the parameter type her from &str to the generic T to have the flexibility that we want
        Self {
            name, // use shorthand syntax here to assign whatever is provided in the name parameter to the name field
            reservation: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description()) // because T is a generic, Rust cannot guarantee that it will implement the display {} trait
                                                            // We can solve this problem with an trait bound, an additional constraint on the generic type
    }
}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) -> () {
        self.reservation.insert(name.to_string(), nights); 
    }
}

impl<T> Description for Hotel<T> {} 

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) -> () {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("{} is the best host we have ever had", self.host)
    }
}


fn book_for_one_night(entity: &mut (impl Accommodation + Description), guest: &str) {
    entity.book(guest, 1);
}

fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)  
where 
    T: Accommodation + Description,
    U: Accommodation
{
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 2);
}

fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Luxe")
}

fn main() {
    let hotel1 = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel1.summarize()); // Can call this method because the generic T in this case implements the Display Trait

    let hotel2 = Hotel::new("The Golden Standard");
    println!("{}", hotel2.summarize()); // String slice will also support the summarize method

    let hotel3 = Hotel::new(vec!["The Sweet Escape", " The Hills"]);
    //println!("{}", hotel3.summarize()); // This will not compile because the type for T is a vec and does not implement the Display trait
    println!("{:?}", hotel3);
}
