/* Traits as Function Return Types
    We can set the Return Type of a Function to be any Type that Implements a specific Trait or Traits
*/

use std::collections::HashMap;

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32) -> ();
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservation: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservation: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    fn book(&mut self, name: &str, nights: u32) -> () {
        self.reservation.insert(name.to_string(), nights); 
    }
}

impl Description for Hotel {} 

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

// Showing both the Trait Bound Syntax and the Where Clause Syntax
//Trait Bound:
// fn mix_and_match<T: Accommodation + Description, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {code body}
// Where Cluase accomplishes the same thing, but with a different syntax
fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)  
where 
    T: Accommodation + Description,
    U: Accommodation
{
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 2);
}

// Traits as a Function Return Type
fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Luxe")
    //Although the return type is any type that implements both of these traits, we can not have more than one type or we will get an error.
}


fn main() {
    let mut hotel = choose_best_place_to_stay();// Analyzer identifies the type as any type that implements the Accommodation Trait, not Hotel Type
    let mut airbnb = AirBnB::new("Peter");   
    mix_and_match(&mut hotel, &mut airbnb, "Penelope");
    //initially mix_and_match would not compile b/c the parameter type requies a type that implements Accommodation and Description Traits, had to change return type
    //of choose_best_place_to_stay to be a type that implements both Accommodation and Description in order to compile

}
