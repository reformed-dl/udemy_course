//Practice with Ownership and String Concatenation
fn main () {
 let mut trip = start_trip();//blank string
 visit_philadelphia(&mut trip);//invoke visit_philadephia fn, pass in &mut trip to reference trip and edit the String, but not pass ownership to city parameter.
 trip.push_str(" and ");//concatenate to the end of the mutable String
 visit_new_york(&mut trip);
 trip.push_str(" and ");
 visit_boston(&mut trip);
 trip.push('.');//concatentate a char to the end of a string
 show_itinerary(&trip);//reference to the trip String passed in as an argument, ownership not transferred
 println!("{}", trip);//trip is still valid and usable

}

fn start_trip() -> String {
    String::from("The Plan is ...")
}

fn visit_philadelphia(city: &mut String) {
    city.push_str("Philadephia");//fn concatenates text to the end of the String
}

fn visit_new_york(city: &mut String) {
    city.push_str("New York");//fn concatenates text to the end of the String
}

fn visit_boston(city: &mut String) {
    city.push_str("Boston");//fn concatenates text to the end of the String
}

fn show_itinerary(itinerary: &String) {
    println!("{}", itinerary);//fn asks for a &String to reference the String and print out results.
}