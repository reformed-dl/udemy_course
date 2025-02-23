fn main() {
  //tuples can be of multiple data types
  let employee: (&str, i32, &str)  = ("Molly", 35, "Engineering");
  println!("Employee Details {:?}", employee); //must use {:?} to print full tuple
  //let name = employee.0;
  //let age = employee.1;
  //let department = employee.2;
  let (name, age, department) = employee;//this is shorthand for the 3 commented out lines above
  println!("Name: {}, Age: {}, Department: {}", name, age, department);

}
