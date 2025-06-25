/* Trait Bounds and Trait Bound Syntax
A trait bound requires that a generic type implement a specific trait
*/

use std::{arch::x86_64::_MM_MASK_DIV_ZERO, collections::HashMap};

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

//Generic Types with Trait Bound Syntax -> <T: Trait Type>
fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

//This syntax allows us to use different types for each of the first and second parameters, as long as each Type implements the Trait
fn mix_and_match(first: &mut impl Accommodation, second: &mut impl Accommodation, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

//This syntax demands that both parameter types, T, must be the same Type, we further constrain the parameter type to the same Type
//This code won't compile, because you cannot borrow as mutable more than once
//fn mix_and_match_2<T: Accommodation>(first: &mut T, second: &mut T, guest: &str) {
   // first.book(guest, 1);
   // second.book(guest, 2);
//}

//This syntax allows us to have the flexibility of the fist function, with the syntax of the second, by declaring different generics
fn mix_and_match_3<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

fn main() {
    let mut hotel = Hotel::new("The Lux");
    let mut airbnb = AirBnB::new("Peter");
   
    mix_and_match(&mut hotel, &mut airbnb, "Penelope");
    println!("{:?} {:?}", hotel, airbnb);

  //  mix_and_match_2(&mut hotel, &mut hotel, "Ricardo"); //will not compile

    mix_and_match_3(&mut hotel, &mut airbnb, "Consuelo");
    println!("{:?} {:?}", hotel, airbnb);

}
