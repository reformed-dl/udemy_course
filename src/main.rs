//Methods on Enums (3) Exact Match
#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
}

impl Milk {
    fn kind(&self) {
        match self {
            Milk::Lowfat(2) => {//determine exact match i32 value
                println!("2% Milk is my favorite");
            }
            Milk::Lowfat(percent) => {//'percent' is whatever name I choose, this will satisfy match for all other i32 values
                println!("{:?} Milk is not good", percent);
            }
            Milk::Whole => {
                println!("Whole Milk has too much fat");
            }
        }
    }
}
   
fn main () {
 Milk::Lowfat(1).kind();
 Milk::Lowfat(2).kind();
 Milk::Whole.kind();
}

    
