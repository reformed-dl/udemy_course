 //Result Enum - Parse Method on a String

fn main () {
/*this example will return a Result Enum. 
We have a string slice "50", but we want to extract the integer equivalent from "50" 
the parse() on a string attempts to extract a different data type
This operation will either succeed or fail with an error. If the string has numeric content within it, we will succeed, if not, we will fail
Successful - return a Result Enum with the Ok variant and the variant will store the extracted integer
Failure - return a Result Enum with the Err variant with it's corresponding error value
Need to use the turbofish operator to specify the of the success value*/
   let text = "50";
   let text_as_number = text.parse::<i32>();//ParseIntError is provided for the Err variant and it it's own specific type
   println!("{:?}", text_as_number);//will print Ok(50)

   let text = "Mississippi";
   let text_as_number = text.parse::<i32>();
   println!("{:?}", text_as_number);//will print Err(ParseIntError { kind: InvalidDigit }) because we were trying to parse a int that wasn't present

 }
