  //functions with parameters and passing in arguments
fn main() {
    open_store("Gainesville");
    baking_pizza(20, "pepperoni");
    making_money();
}

fn open_store (neighborhood: &str) {
    println!("I am going to open a pizza store in {}", neighborhood);
    }
fn baking_pizza (number: u8, toppings: &str) {
    println!("I bake {} pizzas a day on average, with {} toppings on each.", number, toppings);
    }

fn making_money () {
    println!("I am making money hand over fist");
    }
