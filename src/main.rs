//Ownership with Mutable Parameters, Return Values
fn main () {
    let ribs = String::from("smoked ribs");
    lunch(ribs); //call lunch fn, burger variable is passed in as an argument, let meal = ribs, then " and corn on the cobb" is added to the end of the String

    let legs = workout();//legs = implicit return value of the workout() fn
    println!("Set PBs in {} today", legs);
    println!("{}", workout()); //workout() is still usable

    let cake = bake_cake();//cake = explicit return value of the back_cake() fn
    println!("I have a {} cake", cake);
    println!("{}", bake_cake());//bake_cake() is still usable
}

fn lunch (mut meal: String) {
    meal.push_str(" and corn on the cobb"); //take whatever value is given for meal and add " and corn on the cobb" to the end of it
    println!("{} is on the menu for lunch today", meal);
}

fn workout () -> String {
    String::from("Deadlifts and Squats") //this function provides an implicit return of this String Value
}

fn bake_cake () -> String {
    let cake = String::from("Chocolate Mouse");
    return cake; //this function provides an explicit return of the String Value
}