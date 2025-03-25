//If Let Construct Vs While Let Constructs

fn main() {
   let mut sauces = vec!["Mayo", "Ketchup", "Ranch"];

  //if let, hard code Some(sauce) on the left side set equal to the sauce vec on the right. pop() will pull each element from the vec
  //and as long as there is an element there to satisfy the Some(), the print line will be ran
   if let Some(sauce) = sauces.pop() {
    println!("The next sauce is {}", sauce);//Will pop Ranch
   }

   if let Some(sauce) = sauces.pop() {
    println!("The next sauce is {}", sauce);//will pop Ketchup
   }

   if let Some(sauce) = sauces.pop() {
    println!("The next sauce is {}", sauce);//will pop Mayo
   }

   if let Some(sauce) = sauces.pop() {
    println!("The next sauce is {}", sauce);//Some() is no longer satisfied as there are no more elements in the vec, nothing will print out.
   }

   //a more elegant option is a Wile Let construction that will provide the same outputs, but will run as long as the Some() is satisfied
   let mut records = vec!["Rolling Stones", "Led Zepplin", "CCR"];

   while let Some(record) = records.pop() {
    println!("The next record to play is: {}", record);//This will pop from the last element working towards the first and only requires one line of code.
   }
}
