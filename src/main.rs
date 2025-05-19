/*Error Handling-Propagating Errors(Initial Re-factor)
To propagate an error means to bubble/send it up so it is handled by a higher level piece of code
For Example, if a function calls another function, the calling function can handle the errors that the called function generates and sends back up to
the calling function
This allows us to simplify our code and add some flexibility to it

Looking at the code we have written so far, there are some weaknesses
1) The main function is doing a lot of heavy lifting. Usually, it is ideal if main function remains small and kick starts the main logic
and delegates the responsibilities to other smaller functions
2) A lot of repetitions in pattern. 
3) No way to customize these error messages, they are all hard coded

Design we will look at now moves us more towards a better/cleaner direction

We will re-factor the code we have written in the previous lesson*/

use std::fs::File;
use std::io::{self, stdin, Read};//self allows us to pull the Error type living within the "io", input-output, submodule
//use std::process; 

fn main() {
  //We have propogated all the errors from the function below up to main, now we need to handle them
  //We can do that with a match statement on "file_result" which we have set equal to the read_file fn that returns a Result Enum
   let file_result = read_file();
   
   match file_result {
    Ok(conents) => println!("{}", conents),
    Err(error) => {
      eprintln!("There was an error: {}", error)
      //no need to add the process::exit(1) here as the code ends. If we had more code, then we could add the termination code if Err was returned
      //which means we can remove the use std::process; import from the top of the file
    }
  }
}
//Refactor everything by bringing all of the code in main and placing it in the read_file function
//Return Type of Result Enum, because it represents an operation that may succeed or it may fail
//If it succeeds that means we have successfully asked the user for the file to parse, read it, and packed its contents in the Ok Variant
//If any of the 3 operations fail, we can capture the respective error and package that up in the Err Variant 
//Each of the operations return a specific type of error, the io::Error, so that is why we choose that in our Return Type and bring in use std::io{self, stdin, Read};
//Initially Rust is unhappy because we are not returning a Result Enum, we must go through the previous code and correct all specific occurrences
fn read_file() -> Result<String, io::Error> {
  println!("Please enter the name of the file you would like to read");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    //This is the first place where we run into the Err variant from the read_line() method, rather than printing error and terminating, we will propogate up
    //In other words, I am going to send it up to main, so main can handle it
    if let Err(error) = user_requested_file {
        //eprintln!("Something went wrong collecting user input. The erorr was {:?}", error);
        //process::exit(1)
        return Err(error); //using an explicit return to package up the error variable and package it up in th Err Variant and return it, same as writing Result::Err();
    }

    //We will do the same here
    let mut file = match File::open(&input.trim()) {
        Ok(file) => file,
        Err(error) => {
            //eprintln!("Something went wrong opening the file. The error was {}", error);
            //process::exit(1)
            return Err(error);
        }
    };
    
    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);//Returns a Result enum
    //And once more here
    if let Err(error) = read_operation {
      //eprintln!("Something went wrong reading the file. The error was {}", error);
      //process::exit(1)
      return Err(error);
    }

    //Instead of printing here, we will return the Ok Variant
    Ok(file_contents)

}
  
 