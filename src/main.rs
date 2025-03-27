/*Vectors - Ownership
Same rules apply as other types

When borrowing a reference to a vector:
if there ia a mutable reference, there can be no other references, mutable or otherwise
if there is an immutable reference, we can have as many other immutable references as we want

When we insert or remove elements from a vector, behind the scenes that may require new memory allocation on the heap
This can cause issues with the compilier based on certain scenarios*/

fn main() {
   let led_zepplin = String::from("Led Zepplin");
   let ccr = String::from("CCR");
   let pearl_jam = String::from("Pearl Jam");

   let bands = vec![led_zepplin, ccr, pearl_jam];//ownership is moved and variables are no longer usable
   let mut music = bands;//ownership is moved and bands is no longer usable

   let music_reference = &music[2];//this will work here as we have a single immutable reference for the vec
   println!("{}", music_reference);

   music.push(String::from("Cream"));//this is a mutable reference to the music vector and alters the vector
   println!("{:?}", music);
   //now, if I try and use music_reference after this point, I will get an error
   //I can use music_reference up until the point where I use a mutable reference, that is the lifetime of the music_reference variable
   
}
