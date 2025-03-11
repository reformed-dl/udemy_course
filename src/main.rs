//Enums If Let Construct
//Match keyword is exhaustive, it has to cover every possible variant
//we can use the _ wildcard, but if we only have one variant that we want to run code for, we can use if let construct 
#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy{kind: String},
}


fn main () {
    let my_beverage = Milk::Whole; //variable assignment to an enum variant
    //variable is hardcoded now, but if it was dynamically entered and we wanted to only run code if the input met a certain value
    if let Milk::Whole = my_beverage {//opposite construction as we would normally assign variable values
        println!("You have whole Milk");
    }

    let wife_beverage = Milk::Lowfat(2);//define variable and determine i32 value
    if let Milk::Lowfat(percent) = wife_beverage {//percent was determined by me and becomes the variable that is interpolated in println!
        println!("Wife's beverage is {}% milk", percent);//2 is passed in from the associated data of the variant
    }

    let kid_beverage = Milk::NonDairy { kind: String::from("Oat Milk") };//define the String Value in the variant associated data
    if let Milk::NonDairy { kind } = kid_beverage {//'kind' is provided from the struct associated data and becomes what is interpolated in println!
        println!("Kid's beverage is {}", kind);//'Oat Milk' is passed in from the struct associated data of the variant
    }
}

    
