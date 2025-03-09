//Nesting Enums in Enums
#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurantItem {
   Burrito(Meat),//using the enum above as the associated filed for this variant
   Bowl(Meat),
   VeganPlate,
}

fn main() {
    let lunch = RestaurantItem::Burrito(Meat::Steak);
    let dinner = RestaurantItem::Bowl(Meat::Chicken);
    let abandoned_meal = RestaurantItem::VeganPlate;
    println!("Lunch was: {:?}", lunch);
    println!("Dinner was: {:?}", dinner);
    println!("The meal nobody chose was: {:?}", abandoned_meal);

}   
   
