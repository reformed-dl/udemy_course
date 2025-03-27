/*Vectors - Reading Elements and the get()
Same rules apply for copy, borrow and move as with other types
The get() extracts a vector element by index position. It returns an Option Enum, Some() or None*/

fn main() {
   let pizza_diameters = vec![8, 10, 12, 14];

   let pepperoni = String::from("Pepperoni");
   let mushrooms = String::from("Mushrooms");
   let sausage = String::from("Sausage");

   let toppings = vec![pepperoni, mushrooms, sausage];//ownership has moved from the variables to the vec
   println!("{:?}", toppings);
   //println!("{}", pepperoni);//cannot use these variables now that ownership has moved

   let value = pizza_diameters[2];//because i32 implements the copy trait, a full copy is made and no compilier issue
   println!("{}", value);
   println!("{:?}", pizza_diameters);

   //let pizza_topping = toppings[1];//cannot perform this operation because vec contains Strings and Rust will not move ownership for one element of the vec
   //instead we will need to utilize a reference
   let pizza_topping = &toppings[2];
   println!("{}", pizza_topping);

   //slices follow the same syntax as before
   let reformed_pizza = &toppings[..1];
   println!("{:?}", reformed_pizza);
   let gross_pizza = &toppings[1..];
   println!("{:?}", gross_pizza);

   //get()
   let option = toppings.get(0);
   match option {
      Some(topping) => println!("The topping is: {}", topping),
      None => println!("No value at that position"),
   }

   let option_two = toppings.get(3);//get() allows us to avoid a panic
   match option_two {
      Some(topping) => println!("The topping is: {}", topping),
      None => println!("No value at that position"),
   }

   let option_three = toppings.get(1);
   println!("{:?}", option_three);//Some("Mushrooms")

   let option_four = toppings.get(100);
   println!("{:?}", option_four);//None
}
