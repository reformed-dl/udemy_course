/* A Preview of Trait Objects - an instance of a Type that implements a particular trait whose methods will be accessed at runtime using a feature caled dynamic dispatch

    Dynamic Dispatch (dyn): refers to a process where a method will be determined and called at runtime, not compile time. Meaning, Rust will figure out the exact 
    type and the exact method to call at runtime, as opposed to knowing the exact type and method beforehand, aka static dispatch. With static dispatch the compiler
    knows the exact type and the exact methods that will be invoked on that type at compile time. Static is less flexible, but faster. Dynamic Dispatch will only work
    with references, &.

*/

use std::collections::HashMap;
use std::fmt::Display;

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32) -> ();
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}

#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservation: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservation: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
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
    let mut hotel = Hotel::new(String::from("The Luxe"));
    let mut airbnb = AirBnB::new("Peter");

    // Not possible to assign both of these variables to a Vec because Rust will assume we have a consistent type informed by the very first element
    // What we want to do is define the specific type of Vec, but use dyn syntax that will allow any type that implements the Description Trait to be included as valid
    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];
    println!("{}", stays[0].get_description()); // at runtime Rust determines the exact type and method that is being invoked
    println!("{}", stays[1].get_description());

    let mut bookings: Vec<&mut dyn Accommodation> = vec![&mut hotel, &mut airbnb];
    bookings[0].book("Paulie", 3);
    bookings[1].book("Johnny", 4);
    println!("{:?}", hotel);
    println!("{:?}", airbnb);

}
