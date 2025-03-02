/*Creating Structs in a Function
Structs can serve as return values. Moving the Struct outside of the main fn, allows it to be used outside of the scope of main. 
Now that it exists at the file level, it allows us to refer to this struct within any function in this file. More Global, More Available.
 */
struct Coffee {
    price: f32,
    name: String,
    is_hot: bool,
}

fn main() {
    let coffee = make_coffee(String::from("Espresso"), 5.99, true);//because Coffee is returned, it is not lost and coffee becomes the new owner of the values of the struct instance
    println!("My {} cost {} and it is {} that it was hot.", coffee.name, coffee.price, coffee.is_hot);

    let name = String::from("Latte");//This is reassigning the field name values
    let price = 3.99;
    let is_hot = false;
    let latte = Coffee {//calling a new instance of the Coffee Struct
        name,
        price,
        is_hot,
    };
    println!("My {} cost {} and it is {} that it was hot.", latte.name, latte.price, latte.is_hot);

    let coffee = make_coffee(String::from("Espresso"), 5.99, true);//I can still use this here because I am just calling a new instance of the Coffee Struct
    println!("My {} cost {} and it is {} that it was hot.", coffee.name, coffee.price, coffee.is_hot);

}

fn make_coffee(name: String, price: f32, is_hot: bool) -> Coffee {//I can return any valid type, the struct Coffe is now a valid type to be implicity returned
    Coffee {
        name: name,//first name corresponds to the name field in the coffee struct, the second name corresponds to the name parameter that will be passed in
        price: price,//first price corresponds to the price field in the coffe struct, the second price corresponds to the price parameter that will be passed in
        is_hot: is_hot,//the first is_hot corresponds the the field name, the second is_hot corresponds to the parameter that will be passed in
        /*you can use a shortcut for syntax when the stuct field and that parameter names are the same
        name,
        price,
        is_hot,
        and this will work, but the field name and the parameter name must be identical */
    }
}