/// Tools for inventory management
pub mod inventory;
/// Tools for orders management
pub mod orders;

pub use inventory::{Item, ProductCategory, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
pub use orders::MANAGER as ORDERS_MANAGER;