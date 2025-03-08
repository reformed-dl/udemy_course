//Enums a type that represents a set of possible values. It is a set number of possible values. Possible values are referred to as Variants.
//Enums are a specific type and can be used like any other type
#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuit,//using the enum as the field type
}
fn main() { 
    let first_card = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs; //can use variable shadowing, but type must remain the same, going from one enum variant to another
    println!("{:?}", first_card);
    println!("{:?}", second_card);

    let card_array = [CardSuit::Diamonds, CardSuit::Clubs];
    let card_tuple = (CardSuit::Spades, CardSuit::Hearts);

    println!("{:?}", card_array);
    println!("{:?}", card_tuple);
}
