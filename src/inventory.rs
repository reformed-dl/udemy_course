pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory"; //in order to access this const, must use the 'pub' keyword

pub fn talk_to_manager() {
    println!("Hey {} how's your coffee?", MANAGER);
}

pub mod products;
