//HashSet - Distinct Type, but utilizes the HashMap behind the scenes to implement it's functionality. Often a caled a Set in other languages
//A collection type that stores unique values. Solves the problem of duplication. Prohibits duplicate values

use std::collections::HashSet;
fn main() {
  let mut concert_que: HashSet<&str> = HashSet::new();

    concert_que.insert("Molly");
    concert_que.insert("Megan");
    println!("{:?}", concert_que);
    println!("{:?}", concert_que.len());

    concert_que.insert("Molly");
    println!("{:?}", concert_que);//Nothing is added, because it won't allow duplicates

    //remove will remove the value and return a boolean
    println!("{:?}", concert_que.remove("Megan"));
    println!("{:?}", concert_que.remove("Megan"));//already removed so will return false

    //Contains check to see if the argument is contained within the HashSet, returns a boolean
    println!("{:?}", concert_que.contains("Megan"));
    println!("{:?}", concert_que.contains("Molly"));

    //Get returns an Option Enum - Some(&Associated Data) or None
    println!("{:?}", concert_que.get("Molly"));//Some("Molly")
    println!("{:?}", concert_que.get("Megan"));//None







}
