//Nesting Enums in Enums, Enums as Associated Fields.
#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurantItem {
   Burrito{meat: Meat, bean: Beans},//using 2 enums as an associated field, instead of (), use {} and set up as a named field struct
   Bowl{meat: Meat, bean: Beans},
   VeganPlate,
}

fn main() {
    let lunch = RestaurantItem::Burrito { meat: Meat::Steak, bean: Beans::Black };
    let dinner = RestaurantItem::Bowl { meat: Meat::Chicken, bean: Beans::Pinto };
    let abandoned_meal = RestaurantItem::VeganPlate;
    println!("Lunch was: {:?}", lunch);
    println!("Dinner was: {:?}", dinner);
    println!("The meal nobody chose was: {:?}", abandoned_meal);

}   
   
