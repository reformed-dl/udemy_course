 /*Built-In Enums (Using Option Enums with get(), unwrap(), expect())
1 of the 2 most common is the Option Enum
The Option Enum models a scenario where a Type could be a valid value or nothing at all, values present or absent
Other languages represent/model absent values using a null value. However, this null value has long been known to cause errors
Option Enums are Rust's solution to this problem

Option::None represents an absent value
Option::Some(T) represents a present value; where T is the generic data type of the valid value that the Some variant will hold, T is the associated data for the Some Variant*/

fn main () {
    let a = Option::Some(5); //whatever is passed in becomes the concrete type for the generic type T, Rust default int type is i32
    let b = Option::Some("Hello Rust");//&str becomes concrete type for generic type T
    let c = Option::Some(String::from("Goodbye Rust"));//String becomes the concrete type for generic type T

    //to define the specific type and not let Rust choose default
    let d: Option<u8> = Option::Some(5);
    //you can also use the turbofish operator
    let e = Option::<u16>::Some(5);//turbofish inserted between Option and ::Some

    let f: Option<String> = Option::None;//If using Option::None, we do have to explicitly define the concrete type in case ::None becomes ::Some

    //One option to extract an element from an array is to provide index position within [], 
    //However, this can result in errors, especially with user input, a safer method is to use the 'get method' which is an option enum
    let musical_instruments = [String::from("Guitar"), String::from("Drums"), String::from("Bass")];

    let bass = musical_instruments.get(2);//the type for the generic type T is a &String(get() does not extract the string) can also be hardcoded Option<&String>
    println!("{:?}", bass);//result will be Some("Bass"); Option::Some(T); the type returned by the get() is an Option Enum of the Sum Variant storing a &String as the associated data

    //One option to extract the data from a Option Enum is the unwrap method. Unwrap Method attempts to extract the associated data out of the Some variant
    //This will work for Option::Some(T), however it will result in a runtime panic error if we have Option::None
    let valid_instrument = bass.unwrap();//Since bass is Option::Some(T) -> Option<&String> -> unwrap will extract the associated data
    println!("{}", valid_instrument); //result will be 'Bass"

    let drums = musical_instruments.get(1);
    println!("{:?}", drums);//result will be Some("Drums"); Option::Some(T)

    //Another option to extract the data from an Option Enum is the expect method. Expect will provide the same results as unwrap on any Option::Some. 
    //However, expect() allows us to customize our error message on Option::None
    let valid_drums = drums.expect("Unable to retrieve element");//error message will only run if associated data not found, Option::None
    println!("{}", valid_drums);//result will be 'Drums'

    let invalid_instrument = musical_instruments.get(100);//if attempted with [] would cause a panic at run time
    println!("{:?}", invalid_instrument);//result will be None, instead of a panic error; Option::None
    //unwrap() will not work on Option::None, and will result in a panic at runtime
    invalid_instrument.expect("Unable to retrieve element");//result will be 'Unable to retrieve element'

 }
