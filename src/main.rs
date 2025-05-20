/*Error Handling-The ? Operator (The Try Operator)
We established a pattern of propagating errors up to the calling function, so the caller, in this case main, could handle the specifics of the error
This pattern is so common that Rust provides an operator that performs the exact same logic, the ? operator
We apply the ? operator after a Result and it works the exact same way as the previous code written
If the Result Enum Variant is the Ok Variant, then the ? produces the associated data that is stored within the Ok variant associated data, the fn
continues executing

If the Result Enum Variant is the Err Variant, then the ? returns the error as the functions return value and the function terminates earlier

We will refactor the read_file fn using the ? operator
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

    //let user_requested_file = stdin().read_line(&mut input);
    //We no longer need the user_requested_file variable anymore
    //Simply going to call the read_line method because it returns a Result
    //I either want to progress forward with the Ok variant data or return early with the Err Variant

    stdin().read_line(&mut input)?;//the ? accomplishes the exact same lines of code below and they can be removed

    //if let Err(error) = user_requested_file {
       // return Err(error);
    //}

    //We can remove the match and the ? accomplishes the Ok and Err variant match arms below
    let mut file = File::open(&mut input.trim())?;

   // let mut file = match File::open(&input.trim()) {
       // Ok(file) => file,
       // Err(error) => {
          //  return Err(error);
       // }
   // };
    
    //we can do the same here, remove the read_operation variable and use the ? operator on read_to_string()
    let mut file_contents = String::new();
   // let read_operation = file.read_to_string(&mut file_contents);
    file.read_to_string(&mut file_contents)?;
   // if let Err(error) = read_operation {
    //  return Err(error);
   // }

   //I can refactor even further, by stringing methods together. Instead of the code above, I can remove the let mut file variable and instead write
   // let mut file_contents = String::new();
   // File::open(&mut input.trim())?.read_to_string(&mut file_contents)?;
   // we can do this because the only reason why we needed the file variable is so we could call read_to_string() on the file
   // both open and read_to_string return a Result type
   // the ? will either return early or produce the actual File struct, it is that File struct that has the read_to_string() method, so I can add that on
   // File::open(&mut input.trim())?.read_to_string(&mut file_contents)?;

    Ok(file_contents)

}
  
 