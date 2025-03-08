//Enums with Associated Values
#[derive(Debug)]
enum PaymentMethodType {//can store one or more pieces of associated data with enum variants, define the types not the concrete values
    CreditCard(String),//called tuple variants
    DebitCard(String),
    Bitcoin(String, String),
    Cash,
}
fn main() { 
    let visa = PaymentMethodType::CreditCard(String::from("4476-8723"));
    let mastercard = PaymentMethodType::DebitCard(String::from("6879-2145"));
    let bitcoin = PaymentMethodType::Bitcoin(String::from("Multi-Sig Custody"), String::from("Not Your Keys, Not Your Cheese"));
    let fiat = PaymentMethodType::Cash;

    println!("{:?}", visa);
    println!("{:?}", mastercard);
    println!("{:?}", bitcoin);
    println!("{:?}", fiat);

}
