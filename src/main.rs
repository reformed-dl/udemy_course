/*Module: Organizational containter that encapsulates related code. Similar to a Folder on Your Computer
 keyword is "mod" and name is written in snake case
 Modules allow us to have duplicate names. Each duplicate name can live within it's own Modules, it's own Namespace
 
 There are multiple (3) ways to declare a Module
 1) By declaring the mod keyword and all of the associated data within the curly braces
 2) By creating a file within the src folder. 
      Example inventory.rs and then any code written within that file can be accessed by typing "mod inventory;" in main
 3) By creating a directory(folder): create a subfolder within src folder with the module name and a file within the sub folder named "mod.rs*/

 mod inventory;
 mod orders;

fn main() {
   println!("Our managers are {} and {}, we have {} square feet of floor space.", inventory::MANAGER, orders::MANAGER, inventory::FLOOR_SPACE);

   inventory::talk_to_manager();

   let favorite_category = inventory::ProductCategory::Hammer;
   println!("My favorite category of item is {:?}", favorite_category);

   let tall_ladder = inventory::Item {
      name: String::from("Ladder-o-matic 2000"),
      category: favorite_category,
      quantity: 100,
   };
   println!("{:?}", tall_ladder);
}
