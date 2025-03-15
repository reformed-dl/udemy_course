//Generics in Enums
#[derive(Debug)]

enum Cheesesteak<T> {
    Plain,
    Topping(T),
}
 fn main () {
    //a few declarations and how they work with Enums and Generics
    let peppers = Cheesesteak::Topping("peppers");
    let cheesy = Cheesesteak::Topping(String::from("provolone and cheddar"));
    let mushrooms = Cheesesteak::Topping("mushrooms".to_string());//turns &str into a String
    let topping = "bacon".to_string();
    let bacon = Cheesesteak::Topping(&topping);//& to the variable above
    //let plain = Cheesesteak::Plain; this will not compile eventhough we are using the Plain variant. Must declare the type T with a concrete type even if not using the Topping variant
    let plain: Cheesesteak<String> = Cheesesteak::Plain;//We will now be held accountable to the String Type, see example below
    let mut plain_two: Cheesesteak<String> = Cheesesteak::Plain;
    plain_two = Cheesesteak::Topping(String::from("No Toppings"));//cannot use a different type here, now that String was declared
 }
