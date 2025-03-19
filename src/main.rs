 /*Top-Level Option Variants
 Rust Prelude: a collection of named constructs that are available automatically in every program. All the top level names availabe to us
 Prelude includes the Option Enum and is available automatically
 Option Enum is so common that we have a shorthand syntax we can use. Instead of Option::Some, we can write Some, instead of Option::None, we can write None*/

fn main () {
    let item_is_available  = is_item_in_stock();

    match item_is_available {
        Some(true) => println!("Yes, the item is in stock and available"),
        Some(false) => println!("No, the item is not in stock"),
        None => println!("We are unable to find the item you are looking for"),
    }
    
 }

fn is_item_in_stock() -> Option<bool> {//Option must remain here in the return type because we are specifying the complete Option Enum as a return type
    let item_exists_in_catalog = true;
    let item_is_in_stock = false;

    if item_exists_in_catalog && item_is_in_stock {
        Some(true)
    } else if item_exists_in_catalog && item_is_in_stock {
        Some(false)
    } else {
        None
    }
}