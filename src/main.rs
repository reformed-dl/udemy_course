 /*The unwrap_or()
 functions similar to unwrap(), but provides a default or back-up value in the case that we have a Option::None
 Trying to call unwrap() on a None variant would provide a panic at runtime*/

fn main () {
    let present_value = Some(13);
    let missing_value: Option<i32> = None;//remember, I have to hardcode the Option<T> with None

    println!("{}", present_value.unwrap_or(0));//the default is the required parameter in case there isn't a Some(T) 
    println!("{}", missing_value.unwrap_or(0));//tells rust if we have an Option with the None variant and there is no associated data to extract, fall back to the default value

    let valid_test = Some("I am coding today");
    let non_valid_test: Option<&str> = None;

    println!("{}", valid_test.unwrap_or("I am not coding today"));
    println!("{}", non_valid_test.unwrap_or("I am not coding today"));

 }
