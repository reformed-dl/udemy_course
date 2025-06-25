/* Using a Trait to add constraints to a functions parameters. 
*/

use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32) -> ();
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
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name) 
    }

    fn book(&mut self, name: &str, nights: u32) -> () {
        self.reservation.insert(name.to_string(), nights); 
    }
}

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
    fn get_description(&self) -> String {
        format!("{} is the best host we have ever had", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) -> () {
        self.guests.push((name.to_string(), nights));
    }
}

/* We can make a function accept any Type that implements a given Trait. We don't declare the Type, rather
we declare the specific Trait and the function knows the parameter will be of a Type that implements that Trait. The function will also be able to invoke any methods
defined in the Trait, regardless of what parameter ends up being */
fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);
}


fn main() {
    let mut hotel = Hotel::new("The Lux");
    book_for_one_night(&mut hotel, "Johnny");
    println!("{:?}", hotel);

    let mut airbnb = AirBnB::new("Peter");
    book_for_one_night(&mut airbnb, "Tommy");
    println!("{:?}", airbnb);
    
}
