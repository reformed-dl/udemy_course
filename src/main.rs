/* Structs Basics
Structs are containers used to store related data
3 types of Structs: 1. Named Field 2. Tuple-like 3. Unit-like
Struct Fields are identified by their name and they have a corresponding value
fields are the owners of their values, structs are the owner of the fields
First defifne a blueprint of the struct and then an instance of the struct in the actual program*/

fn main() {
    struct CoffeDrink {//This is the Bluepring of the Struct we will instantiate in the program
        name: String,//field names and the types of data that will correspond to each field
        price: f32,
        is_hot: bool,
    }

    let americano = CoffeDrink {//This is creating an instance of the Struct
        name: String::from("Americano"),
        price: 5.99,
        is_hot: true,
    };//need an ';' here
    //access a struct field by struct_name.field_name
    println!("{}", americano.name);
    println!("{}", americano.price);
    println!("{}", americano.is_hot);
    println!("This morning I picked up an {} for {}. It is {} that it was hot.", americano.name, americano.price, americano.is_hot);

    //let favorite_drink = americano.name; //ownership is moved here, since name field is a String and stored on the heap, name is no longer viable and usble
    //println!("{}", favorite_drink);

    //Overwriting Struct Fields
    struct ActionHero {//template is not mutable or immutable
        name: String,
        display_character: char,
        starting_lives: u8,
    }

    let mut terminator = ActionHero {//the instance itself needs to be declared mutable, if mutable all fields are mutable. All or nothing
        name: String::from("Terminator"),
        display_character: '@',
        starting_lives: 8,
    };
    println!("My fovrite video game is {}, the character on the map is shown as {} and you have {} starting lives.", terminator.name, terminator.display_character, terminator.starting_lives);

    terminator.name = String::from("Arnold");//access the existing struct name and field. struct_name.field_name and set it to new value
    terminator.display_character = '$';
    terminator.starting_lives = 5;

    println!("{}, {}, {}", terminator.name, terminator.display_character, terminator.starting_lives);
}
