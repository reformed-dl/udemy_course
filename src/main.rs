//If, Else If, Else
//If statements must evaluate to a boolean, each If statement is evaluated individually
//Else If statements stop checking once a boolean has evaluated to true
//Else captures any other value
fn main () {
    let number = 3;
    if number > 4 {
        println!("Number is greater than 4");
    }
    if number < 4 {
        println!("Number is less than 4");
    }

    let season = "potato";
    if season == "summer" {
        println!("The season of sun is here");
    } else if season == "winter" {
        println!("It is getting cold outside");
    } else if season == "spring" {
        println!("Flowers give me allergies");
    } else {
        println!("Who knows??!!");
    }
}

