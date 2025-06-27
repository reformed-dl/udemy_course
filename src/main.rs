/* Multiple Trait Bounds - Requiring multiple traits either for a parameter or bound to a generic
*/

use std::collections::HashMap;

//Splitting existing Accommodation Trait into 2 separate Traits, Accommodation and Description
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
// Must also define an implementation for the Description Trait
impl Description for Hotel {} // leaving the code block empty to it falls back to what was defined in the trait implementation "A wondeful place to stay"

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
// Must also define the implementation for Description Trait here for AirBnB
impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("{} is the best host we have ever had", self.host)
    }
}

// Syntax for Multiple Traits with Generic Trait Bound Syntax, <T: Trait1 + Trait2>
fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

// syntax for Multiple Traits with parameters -> 'first parameter' must accepty a Type that implements both the Accommodation and Description Traits
// (first_param: &mut (impl Trait1 + Trait2), second_param: type)
fn mix_and_match(first: &mut (impl Accommodation + Description), second: &mut impl Accommodation, guest: &str) {
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 2);
    //second.get_description(); this will not work because Rust cannot guarantee the the second parameter has a get_description method
}


fn main() {
    let mut hotel = Hotel::new("The Lux");
    let mut airbnb = AirBnB::new("Peter");
   
    mix_and_match(&mut hotel, &mut airbnb, "Penelope");
    println!("{:?} {:?}", hotel, airbnb);
}
