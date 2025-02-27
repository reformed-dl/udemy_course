//Ownership, Copy, Move and String Concatenation
fn main () {
    let name: String = String::from("Tyson"); 
    println!("{}", name);
    let boxer = &name;//&references name, address to the same piece of data on the heap. Both name and boxer are valid
    println!("{}", boxer);
    println!("{}", name);
    let fighter = name;//Ownership is MOVED from name to fighter. Name is no longer valid after this point, nor is boxer
    println!("{}", fighter);    
    //println!("{}", boxer);
    //println!("{}", name);

    //slices of Strings
    let s: String = String::from("Mighty Mouse");
    let t = &s;
    let v = &s[0..6];
    let z = &s[7..];
    println!("{}", s);
    println!("{}", t);
    println!("{}", v);
    println!("{}", z);

    //Concatenation on Strings
    let mut family: String = String::new();
    family.push_str("Father");
    println!("{}", family);
    let woman: String = String::from(", Mother");
    family.push_str(&woman);
    println!("{}", family);
    let kids = " and Children";
    family.push_str(kids);
    println!("{}", family);

    //Copy Vs Move in fn parameters
    let value = 21;
    immutable(value);//let num = value; COPY is made. 
    println!("{} value is still useable", value);

    let nation = String::from("United States");
    dynamic(nation);//let country = nation; Ownership is moved.
    //cannot use nation as it is no longer valid

    let city = String::from("Washington DC");
    dynamic_two(&city);//parameter asks for a ref to a String, no copy is made, city is still useable.
    println!("{} is still useable", city);

    let king = String::from("Christ");
    dynamic_three(king.clone()); //using .clone() method makes a copy and king is still useable.
    println!("{} is still useable", king);

}

fn immutable (num: i32) {
    println!("{} is an immutable data type", num);
}

fn dynamic (country: String) {
    println!("{} is where I live", country);
}

fn dynamic_two (place: &String) {
    println!("{} is the capital of the USA", place);
}

fn dynamic_three (lord: String) {
    println!("{} is King of Creation", lord);
}