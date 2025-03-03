//String values in Structs are still stored on the heap and you have to be careful for ownership issues.
struct Coffee {
    price: f32,
    name: String,
    is_hot: bool,
}

fn main() {
    let coffee = make_coffee(String::from("Espresso"), 5.99, true);

    let mocha = Coffee {
        name: String::from("Mocha"),
        ..coffee//this tells Rust to copy the values from the coffee variable
    };

    let coffee_two = Coffee {
        ..coffee//thsi copies all of the values in the coffee instance of the struct, and since we have a String, will make coffee.name no longer usable
    };

    println!("{}", mocha.name);
    println!("{}", coffee_two.name);
    //println!("{}", coffee.name); can't use coffee here becuase the name value moved to coffee_two

    //in order to continue to use the string value of an instance after copying the values, use clone()

    let coffee_three = Coffee {
        name: coffee_two.name.clone(),
        ..coffee_two
    };
    println!("{}", mocha.name);
    println!("{}", coffee_two.name);
    println!("{}", coffee_three.name);   
}

fn make_coffee(name: String, price: f32, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}