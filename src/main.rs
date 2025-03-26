/*Vectors - data structure that is similar to an Array. Arrays are of a homogenous type, defined order and a fixed size. The number of elements cannot change.
Vectors allow us to have contiguous, homonegous elements, but the number of elements can change
Vectors are stored on the heap at runtime

Vec is the type and is included in the Rust Prelude
vectors implement the debug trait*/

fn main() {
   let pizza_diameters: Vec<i32> = Vec::new();//will return a new empty Vector, but must provide specific type annotations
   let ball_diamters = Vec::<i32>::new();//can also define specific type annotation using the turbofish operator
   println!("{:?}", pizza_diameters);//output will be []
   println!("{:?}", ball_diamters);

   //if we know the initial values that we start with, we use the vec macro
   let records = vec!["Rolling Stones", "Doors", "Lynard Skyndard"];
   println!("{:?}", records);

   //push() adds a single element to the end of the vector
   let mut food = vec!["Pasta", "Steak", "Ribs"];
   println!("{:?}", food);
   food.push("Chicken Wings");
   println!("{:?}", food);

   //insert() allows to add an element at a certain index position
   food.insert(1, "Mac and Cheese");
   println!("{:?}", food);

   //pop() attempts to remove the last element from the vec and return it
   //pop() is an Option Enum, Some() and None
   let meal = food.pop();
   println!("{:?}", meal);//output will be Some("Chicken Wings")
   println!("{:?}", food);//one pop() has been called, the last element is now removed

   //remove() removes an element by the index position
   food.remove(0);//if index does not exist, will panic
   println!("{:?}", food);//first element has now been removed from the vector

}
