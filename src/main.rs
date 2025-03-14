//Generics is Structs

#[derive(Debug)]
struct TreasureChest<T> {//same set up as a function
    captain: String,
    treasure: T,//allows us flexibility for the Type here, No need to create 3 different Structs, can re-use same struct with generic Type
}
 fn main () {
    let gold_chest = TreasureChest {
        captain: String::from("Captain Sponge Bob Square Pants"),
        treasure: "Gold",
    };
    println!("{:?}", gold_chest);

    let silver_chest = TreasureChest {
        captain: String::from("Captain Second Place"),
        treasure: String::from("Silver"),
    };
    println!("{:?}", silver_chest);


    let mother_load = TreasureChest {
        captain: String::from("Reformed and Salty"),
        treasure: ["Gold", "Silver", "Platinum", "Bitcoin"],
    };
    println!("{:?}", mother_load);

 }
