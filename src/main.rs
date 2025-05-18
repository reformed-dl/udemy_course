/*Error Handling-Reading the File's Contents
so far we have asked the user for the file that they would like to read and we have opened it with File::open
next step is reading the file and displaying it's contents to the user
 */
use std::fs::File;
use std::io::{stdin, Read};//Read allows us to use the read_to_string() method
use std::process;

fn main() {
    println!("Please enter the name of the file you would like to read");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input);
    if let Err(error) = user_requested_file {
        eprintln!("Something went wrong collecting user input. The erorr was {:?}", error);
        process::exit(1)
    }

    let mut file = match File::open(&input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong opening the file. The error was {}", error);
            process::exit(1)
        }
    };
    //on the File struct, which we have as a direct extraction from the Ok variant and assigned to "file", we have a method called read_to_string()
    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);//Returns a Result enum
    //use the same if let syntax to account for the Err Variant
    if let Err(error) = read_operation {
      eprintln!("Something went wrong reading the file. The error was {}", error);
      process::exit(1)
    }

    println!("{}", file_contents);
    /*We have done 3 operations in this code. Each of them Return a Result Enum, because each could result in a error or a failure: 
    Reading the user input
    Opening the file
    Reding the file to a string
    If we received an error at any point, we printed out an error message and terminated the program
    We repeated the if let syntax and error handling so many times and it is so common that rust provides a simplified option that we can use
    Code above is completely valid though */
}
