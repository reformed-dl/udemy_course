//Return Types, Explicit and Implicit
fn main() {
    let total = square(5);
    println!("{}", total);

    let result = cubed(4);
    println!("{}", result);
}

fn square(number: i8) -> i8 {
    //must declare the return type
    return number * number; //explicit returns require the key word 'return' and the line is finished with a ;
}
fn cubed(number: u8) -> u8 {
    //must declare the return type
    number * number * number //implicit returns do not accept ; the very last line is what you want to return and be the output
}
