pub mod products;

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory"; 

pub fn talk_to_manager() {
    println!("Hey {} how's your coffee?", crate::inventory::MANAGER);//this is an example of using the absolute path, from the crate root, using crate keyword
    println!("{:?}", products::ProductCategory::Ladder);//example of relative path, we ae in inventory, then go to products, etc.
}

