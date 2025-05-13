use std::io; //io - input/output data coming in and going out of our program

fn main() {
   //stdin() is a fn that returns a struct also called Stdin and that struct contains a method called read_line
    let mut name = String::new();
    println!("What is your name?");
    io::stdin().read_line(&mut name); //buf parameter accepts a mutable reference to a String
    //what happens here is when ran, output is What is your name?, then user inputs data in the terminal and we store it
    println!("Hello {}", name);

    //since read_line is a Result Enum, a better way to handle both the Ok and Err possibilities is with a match statement
    let mut whole_name = String::new();
    println!("What is your name?");
    match io::stdin().read_line(&mut whole_name) {
        Ok(_) => println!("Hello {}", whole_name.trim()),//trim removes the extra line from the user hitting enter
        Err(message) => println!("There was an error: {}", message),
    }
}
