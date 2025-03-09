//Enums and Match Statements (3)
#[derive(Debug)]
enum LaundryCycles {
    Cold,
    Hot{temp: u32},//struct named field associated value
    Delicate(String),//tuple like associated value
}

fn wash_laundry(temperature: LaundryCycles) {
    match temperature {
        LaundryCycles::Cold => {
            println!("Washing my laundry with cold water");
        }
        LaundryCycles::Hot { temp } => {//here we are extracting our fields by name
            println!("Washing my laundry with {} degree water", temp);
        }
        LaundryCycles::Delicate(fabric) => {//choose whatever name I want to represent the String, here we are extracting our values by order or position
            println!("Washing my {} laundry on the delicate cylce", fabric);
        }
    }
}
fn main () {
    wash_laundry(LaundryCycles::Cold);
    wash_laundry(LaundryCycles::Hot { temp: 100 });//input the 100
    wash_laundry(LaundryCycles::Delicate(String::from("Silk Underoos")));//input the String Value
}
    
