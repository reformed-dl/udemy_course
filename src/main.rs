//Geneics and Impl Blocks
#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}
//One option is to define the Type in the impl block 
//impl TreasureChest<String> {} - this will define methods in this block but only on TreasureChest Structs where the Type of T is a String, no other Type
//In this example this will only the silver_chest struct instance
impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();//Rust knows that self.treasure is a String. 
    }                                                    //This method will trim the white space creating a string slice, the to_string coverts back to a String
}
//the following impl block will only work on the &str array with 4 elements, mother_load struct instance
impl TreasureChest<[&str; 4]> {
    fn amount_of_treasure(&self) -> usize {//whenever we can the len() on an array, we get a usize
       self.treasure.len()
    }
}

//The other option is define a generic type insted of the exact type
//syntax is to identify the generic after impl as well as the Struct Name
//impl<T> StructName<T> {}
//This syntax will allow us to write methods in the impl block that will work on any of the instances
//However, Type Safety should be considered and used whenever possible to avoid compilier errors
impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()//this method will now work on all instances.
    }
}
 fn main () {
    let gold_chest = TreasureChest {
        captain: String::from("Captain Sponge Bob Square Pants"),
        treasure: "Gold",
    };
    println!("{}",gold_chest.capital_captain());
    println!("{:?}", gold_chest);

    let mut silver_chest = TreasureChest {//have to make silver_chest mutable for the clean_treasure method
        captain: String::from("Captain Second Place"),
        treasure: String::from("     Silver     "),//white space will be removed from the clean_treasure method
    };
    silver_chest.clean_treasure();
    println!("{}",silver_chest.capital_captain());
    println!("{:?}", silver_chest);


    let mother_load = TreasureChest {
        captain: String::from("Reformed and Salty"),
        treasure: ["Gold", "Silver", "Platinum", "Bitcoin"],
    };
    println!("{}", mother_load.amount_of_treasure());//provides length of the array, usize
    println!("{}",mother_load.capital_captain());
    println!("{:?}", mother_load);

 }
