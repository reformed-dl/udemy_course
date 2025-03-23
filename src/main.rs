 /*Building an Option from Scratch
 This exercise is designed to re-create the top-level option enum from scratch to see how it works internally*/

 #[derive(Debug, Copy, Clone)]//this emulates the exact functionality that we have on Rust's regular Option Enum
enum MyOption {
    Some(i32),
    None
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh"),//remember that with unwrap() on a None variant, we will get a panic at run time. the panic macro emulates that for us
        }
    }
    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}
fn main () {
    let some_option = MyOption::Some(100);
    println!("{}", some_option.unwrap());//will print out 100
    println!("{:?}", some_option);//will print out Some(100)
    println!("{}", some_option.unwrap_or(0));//will print out 100

    let none_option = MyOption::None;
    println!("{:?}", none_option);//will print out None
    println!("{}", none_option.unwrap_or(0));//will print out the fallback value of 0
    println!("{}", none_option.unwrap());//will print out panic at run time and "Uh oh"
 }
