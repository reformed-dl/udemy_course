/* Creating an Accommodation Trait for Hotel Bookings
*/

use std::collections::HashMap;

trait Accommodation {
    //We are defining which methods a Type MUST implement if they implement the Accommodation Trait
    //Writing the code this way (not defining a code of block in the method) requires any Type that implements the Trait to define the code black when implemented
    /* Another option is defining the body of the method in the Trait, so the Type implementing the trait would just use the generic.
    Example, for get_description, we could write it as follows:
        fn get_description(&self) -> String {
          String::from("A wonderful place to stay")
    } 
    Then the Type implementing the Trait wouldn't have to implement the method and would just use the generic */
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32) -> (); // could also accomplish same with no return type instead of the empty tuple return type
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
    //We can use methods from the defined Trait in other methods within other impl blocks. We cannot add methods to the Trait impl block however as the Trait has a 
    //specific number of defined methods
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    // this is how we implement the Accomodation Trait on Hotel struct. We must implement all Trait items as well
    //Rust analyzer recognizes the parameters and return types we defined above and will automatically provide them when we code the Trait items
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name) // reaching into the Hotel struct and pulling out the name field and returning it with format!
    }

    fn book(&mut self, name: &str, nights: u32) -> () {
        self.reservation.insert(name.to_string(), nights); //using insert() mutates the struct -> &mut self. insert() allows us to insert name and nights for k & v
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

fn main() {
    let mut hotel = Hotel::new("The Lux");
    println!("{}", hotel.get_description());
    println!("{}", hotel.summarize());
    hotel.book("Johnny Rotten", 5);
    println!("{:?}", hotel);

    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Tommy Tunes", 7);
    println!("{:?}", airbnb);
}
