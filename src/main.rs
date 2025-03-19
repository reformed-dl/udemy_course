 /*Match Statements with Option Enums
 Using the unwrap() and expect() methods are optimistic and assume that there is Some variant within our Option Enum and therefore able to extract corresponding
 associated data, but led to a runtime error if the variant was Option::None
 Using Match Statements allows us to deal with both Option::Some(T) and Option::None in a more elegant way as we are required to have match arms that cover all cases*/

fn main () {
    let musical_instruments = [String::from("Guitar"), String::from("Drums"), String::from("Bass")];

    let bass: Option<&String> = musical_instruments.get(2);//Option<&String> is the concrete type since we have a [String; 3] that we are using the get() on and results in a Option Enum

    match bass {
        Option::Some(instrument) => println!("Playing the {}", instrument),//This arm will match. I declare the name for the associated data value 'instrument'
        Option::None => println!("Singing with my voice"),
    }

    play(bass);
    println!("{:?}", bass);//I can still use bass here because Rust implements the Copy Trait on Option Enums

    let invalid_instrument: Option<&String> = musical_instruments.get(100);

    match invalid_instrument {
        Option::Some(instrument) => println!("Playing the {}", instrument),
        Option::None => println!("Singing with my voice"),//This arm will match
    }

    play(invalid_instrument);
    println!("{:?}", invalid_instrument);//I can still use invalid_instrument here as well due to the Copy Trait
 }

 //Instead of writing duplicate code as we have above, we can refactor by writing a function that can be invoked repeatedly
 fn play(instrument_type: Option<&String>) {
    match instrument_type {
        Option::Some(instrument) => println!("Playing the {}", instrument),
        Option::None => println!("Singing with my voice"),
    }
 }
