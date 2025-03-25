//Nuances of Unwrap Method on Enums
//Ownership and Borrowing concerns
fn operation(great_success: bool) -> Result<&'static str, &'static str> {//using this syntax, allows us to avoid ownership issues
    if great_success {
        Ok("Success")
    } else {
        Err("Error")
    }
}

fn main() {
    let my_result = operation(true);//Result<&str, &str> 

    let content = match my_result {
        Ok(message) => message,
        Err(error) => error,
    };

    println!("{}", my_result.unwrap());
    println!("{}", content);
    println!("{}", my_result.unwrap());
}
