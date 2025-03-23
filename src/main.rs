 /*Result Enum
 Option Enums model whether a value is present or absent
 Result Enum modles an evaluation that can produce either a success or a failure(error)
 OK Variant indicates a success and stores and associated piece of data of generic type T
 Err Variant indicated an error and stores an associated piece of date of generic type E
 
 pub enum Result<T, E> {
    Ok(T),
    Err(E),
}*/

fn main () {
    let ok: Result<i32, &str> = Result::Ok(5);//have to provide the type annotation for Err in order for this code to compile
    println!("{:?}", ok);//will print Ok(5)
    let disaster: Result<i32, &str> = Result::Err("Something went wrong");//have to provide the type annotation for Ok in order for this code to compile
    println!("{:?}", disaster);//will print Err("Something went wrong")

    let valid: Result<&str, i32> = Ok("We have a soultion!");//we can drop the Result::, but need to keep it in the concrete type declarations
    println!("{:?}", valid);
    let invalid: Result<&str, i32> = Err(0);//We can drop the Result::, but need to keep it in the concrete type declarations
    println!("{:?}", invalid);

 }
