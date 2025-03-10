//Methods on Enums (2) Matches with different Operators
#[derive(Debug)]
enum OnlinOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlinOrderStatus {
    fn check(&self) {
        match self {
            OnlinOrderStatus::Ordered | OnlinOrderStatus::Packed => {// the | is an or operator
                println!("Your order is being prepped for delivery");
            }
            OnlinOrderStatus::Shipped => {
                println!("Your Order has been shipped");
            }
            _ => {
                println!("Your order has been delivered");// the wildcard catches all other options
            }
        }
    }
}

#[derive(Debug)]
enum GameDevelopment {
    BrainStorming,
    Planning,
    Development,
    Distribution,
}

impl GameDevelopment {
    fn status(&self) {
        match self {
            GameDevelopment::Distribution => {
                println!("Game is Ready! Enjoy Nerds!");
            }
            other_status => {
                println!("Please check back later, game is currently in {:?} phase.", other_status);//other_status will interpolate the enum variant
            }
        }
    }
}
   
fn main () {
    OnlinOrderStatus::Ordered.check();
    OnlinOrderStatus::Packed.check();
    OnlinOrderStatus::Shipped.check();
    OnlinOrderStatus::Delivered.check();
    GameDevelopment::BrainStorming.status();
    GameDevelopment::Planning.status();
    GameDevelopment::Development.status();
    GameDevelopment::Distribution.status();
}

    
