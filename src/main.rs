//Multiple Generics

//Combining Generic and Concrete Parameter Types
fn make_tuple<T>(first: T, second: u8) -> (T, u8) {
    (first, second)
}

//Combining Multiple Generic of the Same Parameter Types
fn make_second_tuple<T>(first: T, second: T) -> (T, T) {//T is a community convention, Rust recognizes that whatever first is, second must be as well 
    (first, second)
}

//Combining Multiple Generics of Different Parameter Types
fn make_third_tuple<T, U>(first: T, second: U) -> (T, U) {//T and U are up to us, but allows us to have multiple different types, but can be the same
    (first, second)
}
 fn main () {
    println!("{:?}", make_tuple("Hello, World", 57));//argument passed in for first can be whatever we want, second is the concrete type

    println!("{:?}", make_second_tuple("Hello, World", "I am a Rust Noob"));//Types must match here as they are both type T

    println!("{:?}", make_third_tuple("Hello, World", 57));//Types can be different 
    println!("{:?}", make_third_tuple("Hello, World", "I am a Rust Noob"));//Types can be the same
 }
