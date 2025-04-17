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
  println!("The manager of our inventory is {}", inventory::MANAGER);//must use "::" to access MANAGER from within the inventory module
  println!("The manager of our orders is {}", orders::MANAGER);
}
