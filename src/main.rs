/*Error Handling

eprintln!() is a macro that prints messages to the standard error, println!() prints messages to the standard output. Think of standard output and
standard error as different tv channels. We can chooses which channel to send different outputs to.

To create a txt file from the terminal in VS Code, cargo run > filename.txt, anything in the compiler that prints to the standard output will
print to the txt file, anything that prints to the error file will terminal */

use std::fs::File; //we are accessing the standard library, fs is the file system module, File is a struct that models a file that we can read/write
                   //This struct contains an Associated function called Open, which is a Result Enum
                   //Success = Ok->associated data is the File struct;
                   //Failure = Err -> associated data is error type that holds specific information about the error that occurred
use std::process;

fn main() {
    let file = File::open("story.txt"); //Since story.txt already exists within the same project, we don't have to provide the full path
    println!("{:#?}", file); //Returns Ok with the entire File Struct which stores fields about the fie: path, read: bool, write: bool, etc

    let nonsense = File::open("nonsense.txt");
    println!("{:#?}", nonsense); //Returns Err Os Struct { code: 2, kind: NotFound, message: "No such file or directory", }

    //Using a match statement to account for both possibilities
    let next_file = match File::open("story.txt") {
      Ok(next_file) => next_file,//Ok arm is matched and this is returned and assigned to the next_file variable
      Err(error) => {
        eprintln!("Something went wrong reading the file. The error was {:?}", error);
        process::exit(1)//since we matched on the Ok arm, no termination and code continues
      }
    };
    println!("{:#?}", next_file);//File Struct is printed   

    let new_file = match File::open("nofile.txt") {
        Ok(new_file) => new_file,
        Err(error) => {
            eprintln!("Something went wrong reading the file. The error was {:?}", error); //error message will be sent to the error output
            process::exit(1) //program will terminate
        }
    };
    println!("{:#?}", new_file);//this code is never ran

}
