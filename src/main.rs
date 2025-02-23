//Assigning a variable to an If, Else If statement
fn main () {
    odd_or_even(17);
    odd_or_even(100);
}

fn odd_or_even (number: u8) {
    let result = if number % 2 == 0 {
       "even"
    } else {
        "odd"
    };
    println!("The number is {}", result);

}
