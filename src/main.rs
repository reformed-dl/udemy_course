fn main() {
  //Arrays are fixed data type as well as fixed size(number of elements)
  let numbers= [24, 56, 2, 89, 100];
  println!("My numbers array: {:?}", numbers);//to print the full array, must be in debug {:?}

  let apples= ["Granny Smith", "Red Delicious", "Honey Crisp"];
  println!("My favorite apple type is: {}", apples[2]);//index position starts at 0
  println!("The length of my apple array is {}", apples.len());//length method added

  let seasons= ["Summer", "Spring", "Winter", "Fall"];
  let first = seasons[0];
  let second = seasons[1];
  println!("The first season is {}, the second is {}", first, second);

  let mut seasons= ["Summer", "Spring", "Winter", "Fall"];
  println!("{}", seasons[3]);
  seasons[3] = "Autumn"; //changing out the 3rd element in the array
  println!("{}", seasons[3]);   
}
