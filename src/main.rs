 //Result Enums as a Return Type
 //Methods on Result Enums

 fn divide(num: f64, den: f64) -> Result<f64, String> {
  if den == 0.0 {
    Err("Cannot divide by zero".to_string())
  } else {
    Ok(num/den)
  }
 }

fn main () {
  let result = divide(18.0, 3.0);//print out will be Result is: 6
  match result {
    Ok(calculation) => println!("Result is: {}", calculation),
    Err(message) => println!("{}", message),
  }

  let result_two = divide(18.0, 0.0);
  match result_two {
    Ok(calculation) => println!("Result is: {}", calculation),
    Err(message) => println!("{}", message),
  }

  let result_three = divide(18.0, 2.0);
  println!("{}", result_three.unwrap());//will extract the piece of data

  let result_five = divide(24.0, 0.0);
  println!("{}", result_five.unwrap_or(0.0));//the or() must contain the same type as the Ok variant
  
  let result_four = divide(18.0, 0.0);
  println!("{}", result_four.expect("cannot divide by zero"));//panic but will provide message

  //is_ok() and is_err() are methods that will return bool values

 }
