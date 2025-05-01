use fake::{Fake, Faker};

use::udemy::ProductCategory;

/// Create a fictional product category
fn main() {
    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);
}