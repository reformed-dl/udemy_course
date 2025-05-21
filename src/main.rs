/*Error Handling- Further refactor
*/

use std::fs::File;
use std::io::{self, stdin, Read};


fn main() {
  
   let file_result = read_file();
   
   match file_result {
    Ok(conents) => println!("{}", conents),
    Err(error) => {
      eprintln!("There was an error: {}", error)
    }
  }
}

fn read_file() -> Result<String, io::Error> {
  println!("Please enter the name of the file you would like to read");
  
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    let mut file_contents = String::new();
  
    File::open(&mut input.trim())?.read_to_string(&mut file_contents)?;
   // The only reason why we needed the file variable is so we could call read_to_string() on the file
   // both open and read_to_string return a Result type
   // the ? will either return early or produce the actual File struct, it is that File struct that has the read_to_string() method, so I can add that on
   // File::open(&mut input.trim())?.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}
  
 