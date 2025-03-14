/*Generics are a Type Argument.
An abstract placeholder type that stands in for a future concrete type
Think about parameter names in functions. They add flexibility for something concrete that will be passed in later. 
fn add_5 (value: i32) -> i32 {
    value + 5
} value name parameter for an i32 Type. we don't know what the argument will be for the value parameter, 
 but the parameter name allows us define a placeholder for an expected future input of any i32 value.
 Generics function in a similar way but for Types not a value.
 
 Turbofish Operator ::<Type>, when invoking a function with a generic type, can include the Turbofish operator to specify the exact type. This can help ensure type safety*/

 #[derive(Debug)]
 struct DeliSandwich {
     name: String,
     price: f32,
 }
 fn main () {
     println!("{}", identity(5));
     println!("{}", identity::<u8>(5));//Turbofish to specify u8 Type
     println!("{}", identity(13.14));
     println!("{}", identity::<f32>(13.14));//Turbofish to specify f32 Type
     println!("{}", identity("hello"));
     println!("{}", identity(String::from("hello")));
     println!("{}", identity(true));
     println!("{:?}", identity(DeliSandwich{name: String::from("Reuben"), price: 5.99}));
     println!("{:?}", identity::<DeliSandwich>(DeliSandwich{name: String::from("Reuben"), price: 5.99}));//Turbofish to specify DeliSandwich Struct Type
 
 
 }
 
 fn identity_i32(value: i32) -> i32 {//can only receive an i32 argument, because we have hard coded the type
     value
 }
 
 fn identity_bool(value: bool) -> bool {//can only receive a bool argument, because we have hard coded the type
     value
 }
 //the syntax to declare a Generic is <WhateverNameWeWant>, community convention is T
 //this syntax tells rust that there will be some Type that will be identified when the function is invoked
 fn identity<T>(value: T) -> T {
     value
 }
    

    
