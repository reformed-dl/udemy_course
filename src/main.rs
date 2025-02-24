//Match Statements
fn main () {
    let evaluation = true; //simple match statement, checking 'evaluation' value and then matching to the match arm
    match evaluation {
        true => {
            println!("This evaluation is true");
        }
        false => {
            println!("This evaluation is false");
        }
    }

    let number = true;
    let total = match number {
        true => 20,
        false => 40,
    };
    println!("{}", total); //The above is evaluating whether 'number' is true or false. It then assigns the value of true or false to the 'total' variable.

    //utlizing the previous else if season statement to compare
    let season = "autumn";
    if season == "summer" {
        println!("School is Out!");
    } else if season == "spring" {
        print!("Birds are singing");
    } else if season == "winter" {
        println!("Brrr so cold");
    } else {
        println!("Lots of Rain");
    }

    match season {
        "summer" => println!("School is Out"),
        "spring" => println!("Birds are singing"),
        "winter" => println!("Brrr so cold"),
        _ => println!("Who Knows!"), //the wildcard must go at the end, match statements function off the first match, if wildcard is placed first, nothing else is checked
    }

    let number = 8;
    match number {
        2 | 4 | 6 | 8 => println!("{} is even", number), // single | checks each number separately
        1 | 3 | 5 => println!("{} number is odd", number),
        _ => println!("Unknown for now"),
    }

    let digit = 6;//using an if statement within a match statement. Defining 'character' to digit and then running the logic
    match digit {
        character if character % 2 == 0 => println!("{} is an even number", character),
        character if character % 2 != 0 => print!("{} is an odd number", character),
        _ => println!("Unknown"), //instead of using the last line with a wildcard, if we know that one of the other match arms will 100% match, can input _ => unreachable!(),
    }
    //Traffic Light Match Statement
    println!("{}", traffic_light("red"));
}

fn traffic_light (light: &str) -> &str {
    match light {
        "red" => "stop",
        "green" => "go",
        "yellow" => "yield",
        _ => "unknown",
    }
}
