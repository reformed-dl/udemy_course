/* Error-Handling - Using ? Operator with Option Enums, Some() - stores 1 piece of associated data, None - stores nothing
If the Option holds the None Variant, then the function will terminate early and return the None variant
If the Option holds the Some Variant, then the associated data will be pulled out and become the value of the expression and the function continues

Very similar to using with Result, if you have Ok or Some Variants, function continues; Err or None, function terminates early
*/

fn main() {
  //In this example we want to calculate the lenghth of the last element in a Vector. Remove the last element and then provide the length
  let mut animals = vec!["Horse", "Giraffe", "Elephant"];
  //now we can call the length_of_last_element function and pass in our Vec
  println!("The length of the last element of the Vector is: {:?}", length_of_last_element(&mut animals));//-> otpt Some(8)
  println!("The length of the last element of the Vector is: {:?}", length_of_last_element(&mut animals));//-> otpt Some(7)
  println!("The length of the last element of the Vector is: {:?}", length_of_last_element(&mut animals));//-> otpt Some(5)
  println!("The length of the last element of the Vector is: {:?}", length_of_last_element(&mut animals));//-> otpt None
  println!("The length of the last element of the Vector is: {:?}", length_of_last_element(&mut animals));//-> otpt None

}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {//this is the type of the data on the Some Variant
  let last_element = input.pop()?;//pop() returns an Option, adding the ? says if Some-Keep going, if None-terminate early
  Some(last_element.len())//manually packaged up last_element.len() into a Option::Some() and set it as the implicit return
}
  
 