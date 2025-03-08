//Enums with Structs as Variants
#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard{username: String, password: String},//can also place the struct directly on the variant like this (this makes it only available on this enum)
    Bitcoin(Credentials),//Using the struct above as my associated value for this variant(advantage is that the struct can be used elsewhere)
    Cash,
}
fn main() {   
    let bitcoin_credentials = Credentials {
        username: String::from("JoeBlow@gmail.com"),
        password: String::from("password lul"),
    };

    let bitcoin = PaymentMethodType::Bitcoin(bitcoin_credentials);

    println!("{:?}", bitcoin);

    let debit_card = PaymentMethodType::DebitCard { username: String::from("JaneDoe@gmail.com"), password: String::from("password2") };
    println!("{:?}", debit_card);
}
