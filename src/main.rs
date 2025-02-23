fn main() {
    //variable delcaration
    let number = 5;
    let touchdown = 7;
    let total = number * touchdown;

    println!("My favorite team scored {} points in the game", total);

    //nested block variable declaration (nested block can pull varaibles from main scope, main scope cannot pull variables from nested block)
    let alpha = 5;
    let beta = {
        let delta = 4;
        delta * alpha
    };
    let gamma = beta * alpha;
    println!("Nested block example total  is: {}", gamma);

    //casting variables
    let miles_away: i32 = 50;
    let _miles_away_i8 = miles_away as i8;
    let miles_away_u8 = miles_away as u8;

    let floating_point: f32 = 100.456;
    let floating_point_u8 = floating_point as u8; //casting from floating point to int type removes the decimals
    println!("The store is {} miles away", miles_away_u8);
    println!("The number is {}", floating_point_u8);

    //raw string declarations
    println!("Dear Emily,\n How have you been?");//\n is a new line
    println!("\tOnce upon a time");//\t is a tab indentation
    println!("Juliet said \"I love you Rome\"");//\"addes quotations to your string"
    let filepath: &str = "\\Documents\\coding_projects\\udemy_course";//\\adds back slashes to your string
    println!("{}", filepath);

    //basic methods
    let value: i8 = -15;
    println!("The absolute value of the number is: {}", value.abs());
    println!("{}", value.pow(2));
    println!("{}", value.pow(3));
    
    let empty_space = "     my content   ";
    println!("{}", empty_space.trim());
    
    //formatting specifiers and methods with floats
    let pi: f32 = 3.14159265;
    println!("{}", pi.floor());//rounds down a float
    println!("{}", pi.ceil());//rounds up a float
    println!("{}", pi.round());//rounds a float based on decimal value
    println!("The value of pi is: {:.4}", pi);//limits the number of decimal places shown

    }


