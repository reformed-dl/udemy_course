/*HashMap::from() is a function that accepts an array of tuples -> [(K, V), (K, V), (K, V)]*/

use std::collections::HashMap;
fn main() {
    let data: [(&str, i8); 3] = [("George", 7), ("Wanda", 4), ("Tommy", 12)];
    let mut employee_data = HashMap::from(data);
    println!("{:?}", employee_data);

    //remove method returns and Option Enum Some()->there is associated data that will be extracted or None->No associated date
    let tommy = employee_data.remove("Tommy");//Tommy is the key and the associated data being extracted is 12
    println!("{:?}", tommy);//returns Some(12)
    println!("{:?}", tommy.unwrap());//returns just 12 without Some(12)


    println!("{:?}", employee_data);//The Key-Value pair of Tommy has now been removed from the HashMap

    let carol = employee_data.remove("Carol");
    println!("{:?}", carol);//Since there is no "Carol" Key, the Option Enum is None
}
