//Methods on Enums (1)
#[derive(Debug)]
enum LaundryCycles {
    Cold,
    Hot{temp: u32},
    Delicate(String),
}

impl LaundryCycles {
    fn wash_laundry(&self) {
        match self {
            LaundryCycles::Cold => {
                println!("Washing my laundry with cold water");
            }
            LaundryCycles::Hot { temp } => {
                println!("Washing my laundry with {} degree water", temp);
            }
            LaundryCycles::Delicate(fabric) => {
                println!("Washing my {} laundry on the delicate cylce", fabric);
            }
        }
    }
}

fn main () {
   LaundryCycles::Cold.wash_laundry();
   let hot_cycle = LaundryCycles::Hot { temp: 100 };
   hot_cycle.wash_laundry();

   let delicate_cycle = LaundryCycles::Delicate(String::from("Silk Underoos"));
   delicate_cycle.wash_laundry();

   LaundryCycles::Hot { temp: 100 }.wash_laundry();
   LaundryCycles::Delicate(String::from("Silk Underoos")).wash_laundry();
}
    
