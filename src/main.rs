/*Error Handling- read_to_string associated function.
This is different from the read_to_string() method that is called on the File struct
the file system module "fs", includes a helpful function called read_to_string that accomplishes everything we want and wrote code for previously
the read_to_string function accepts a file path, attempts to open the file, reads the contents of the file to a heap string and then return the string
packaged up in the Result Enums Ok Variant. If something fails along the way, it instead returns the io:Error type packaged up in the Result Enums Err Variant
*/

//use std::fs::File; we are going to remove File and stop at fs
use std::fs;
//use std::io::{self, stdin, Read}; we no longer need the Read trait either
use std::io::{self, stdin};


fn main() {
  
   let file_result = read_file();
   
   match file_result {
    Ok(contents) => println!("{}", contents),
    Err(error) => {
      eprintln!("There was an error: {}", error)
    }
  }
}

fn read_file() -> Result<String, io::Error> {
  println!("Please enter the name of the file you would like to read");

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    fs::read_to_string(input.trim())//no semicolon for the implicit return

    //the read_to_function associated function basically does everything coded below
    //let mut file_contents = String::new();
    //File::open(&mut input.trim())?.read_to_string(&mut file_contents)?;
   
    //Ok(file_contents)
}
  
 