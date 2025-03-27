/*Vectors - Writing and Overwriting Elements

Vector Capacity - the maximum number of elements that the vector can contain*/

fn main() {
   let led_zepplin = String::from("Led Zepplin");
   let ccr = String::from("CCR");
   let pearl_jam = String::from("Pearl Jam");

   let mut bands = vec![led_zepplin, ccr, pearl_jam];

   bands[2] = String::from("Dire Straits");//this will overwrite an element in the vector
   println!("{:?}", bands);

   let target_band = &mut bands[1];
   target_band.push_str("-Credence Clearwater Revival");//concatenate text to the string
   println!("{:?}", bands);

   let another_band = &bands[0];//I can use a reference here even though there is a mutable reference above, because Rust defined the lifetime of that mutable reference
   println!("{:?}", another_band);//I could not use this before the end of the lifetime of the mutable reference or I would receive a compiler

   //Vector Capacity
   let mut seasons: Vec<&str> = Vec::with_capacity(4);//this will return an empty vector with a maximum number of 4 elements
   println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());//output Length: 0, Capacity: 4

   seasons.push("Summer");
   seasons.push("Fall");
   seasons.push("Winter");
   seasons.push("Spring");
   println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());//output Length: 4, Capacity: 4

   seasons.push("Summer Again");
   println!("Length: {}, Capacity: {}", seasons.len(), seasons.capacity());//output Length: 5, Capacity: 8
   //The capacity increased due to push and the previous allocated memory for the vec with a capacity of 4 is deallocated and a new vec with the capacity of 8 is stored on the heap
   //This is why Rust prohibits more than one mutable reference at a time, so that a reference isn't pointing to something that has been deallocated from memory

}
