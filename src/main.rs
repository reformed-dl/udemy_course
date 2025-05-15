use std::collections::HashMap;
fn main() {
  let mut coffe_pairings: HashMap<&str, &str> = HashMap::new();
  let drink = String::from("Latte");
  let milk = String::from("Oat Milk");
  coffe_pairings.insert(&drink, &milk);
  coffe_pairings.insert("Flat White", "Almond Milk");
  println!("{:?}", coffe_pairings);

  //Every Key must be unique, if we have duplicate keys, the old value will be overwritten by the new value
  coffe_pairings.insert("Latte", "Pistachio Milk");
  println!("{:?}", coffe_pairings);//Previous Key - Value pair has been overwritten

  //Entry Method - Accepts a HashMap Key and returns an Enum called Entry. Entry has 2 Variants, Occupied-the possibility that the given key exists and the
  //Vacant - possibility that the given key does not Vacant variant and will insert the Key and associated Value
  //.or_insert() will only come into play for the Vacant 
  coffe_pairings.entry("Latte").or_insert("Skim Milk");
  println!("{:?}", coffe_pairings);//nothing changes
  coffe_pairings.entry("Americano").or_insert("Whole Milk");
  println!("{:?}", coffe_pairings);//new Key-Value pair inserted

}
