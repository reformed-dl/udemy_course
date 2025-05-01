use fake::Dummy;

/// A category of product that our business sells
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

/// A concrete item in stock within our warehouse
#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

//we can access parent folders/modules by utilizing the super:: keyword
impl Item {
    /// create a new item
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {//fn and methods within a impl block are private by default
        Self { name, category, quantity, }
    }
}