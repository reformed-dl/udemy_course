/*Error Handling-Asking the User for Input
Instead of hard coding the file name, we will ask the user to input the file name they would like to read
As covered previously, in order to collect input we use the the stdin fn from the input/output module (io)
 */
use std::fs::File;
use std::io::stdin;
use std::process;

fn main() {
  //start by giving the user instructions
  println!("Please enter the name of the file you would like to read");
  let mut input = String::new();

  //when stdin() function in invoked, it will return a Stdin struct that contains the read_line() method, which returns a Result Enum
  //user input is stored in "input" variable
  //Using a match statement here works, but the Ok variant is essentially purposeless
  /* match stdin().read_line(&mut input) {
    Ok(size) => println!("The size of the user entry in bytes is {}", size), //Ok variant doesn't stor the user entry, just the size in bytes
    Err(error) => {
      eprintln!("Something went wrong collecting user input. The erorr was {:?}", error);
      process::exit(1)
    }
  } */

  //alternative solution is utilizing the if let syntax. This allows us to cover a specific Enum Variant, only one.
  //we only want to run this code if we in fact have an error, if not, we want to proceed with the remainder of the code

  let user_requested_file = stdin().read_line(&mut input);
  if let Err(error) = user_requested_file {
    eprintln!("Something went wrong collecting user input. The erorr was {:?}", error);
    process::exit(1)
  }
  
  //Now we can run a match statement
  let file = match File::open(&input.trim()) {//trim() is necessary because when user hits enter, the \n linebreak is captured
    Ok(file) => file,
    Err(error) => {
      eprintln!("Something went wrong reading the file. The error was {}", error);
      process::exit(1)
    }
  };
  println!("{:#?}", file);

/*Walking through the logic of this code
println! requests user input
mut input = String::new() provides a variable where user input will be stored
user is prompted to enter file they want to read
if there was an error in collecting user input, Err would be assigned to "user_requested_file" the "if let" syntax would run, error message output, program terminates
if we are able to successfully caputure user input, input will be assigned to "user_requested_file" and "if let" syntax would NOT run
code progresses, we trim the content of what was assigned to "input" and match content and assign results back to "file" variable
If file can be found, matched to Ok arm, file is extracted and assigned back to "file" variable
file variable contents is output 
If file cannot be found, Err arm is matched, error code is output, program is terminated
 */
}
