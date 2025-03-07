//Builder Pattern
#[derive(Debug)]
struct CarLot {
    name: String,
    price: u32,
    miles: u32,
}

impl CarLot {
    fn new(name: String, price: u32, miles: u32) -> Self {//Creates a new instance of the CarLot struct
        Self {
            name,
            price,
            miles,
        }
    }

    fn new_car(&mut self, new_car: String) -> &mut Self {
        self.name = new_car;//overwrites field_value with new value
        self//returns the modifed value
    }

    fn new_price(&mut self, new_price: u32) -> &mut Self {
        self.price = new_price;
        self
    }

    fn new_miles(&mut self, new_miles: u32) -> &mut Self {
        self.miles = new_miles;
        self
    }
}
fn main() { 
    let mut car = CarLot::new(String::from("Ford F-250"), 35000, 10298);//set variable to new instance of the struct
    println!("Car Details: {:#?}", car);

    //builder pattern allows me to string together methods
    //Only possible due to how the methods are designed in the impl block
    car.new_car(String::from("Jeep CJ7")).new_price(25999).new_miles(89200);
    println!("Car Details: {:#?}", car);
}
