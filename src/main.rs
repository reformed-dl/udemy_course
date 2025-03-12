//Enums Let Else Construct
//Opposite contruction to the let if construct
#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy{kind: String},
}


fn main () {
    //Example of when the variable declaration and the let else matches, else block of code will not run
    let wife_beverage = Milk::Lowfat(2);//set variable declaration
    let Milk::Lowfat(percent) = wife_beverage else {//this code will not run and entire block will be skipped because variable declaration is equal to Milk::Lowfat
        println!("You do have whole milk");
        return;
    };

    println!("You have {}% milk", percent);//This code will run and 'perecent' is still usable down here

    //Example of when the variable declaration and the let else does not match, else block of code will run
    let my_beverage = Milk::Whole;//set variable declaration
    let Milk::Lowfat(percent) = my_beverage else {//this code will run because variable does not equal Milk::Lowfat
        println!("You do not have lowfat milk");
        return;//we need to add return; to tell the code to stop running or it will continue and provide an error 
    };

    println!("You have {}% milk", percent);//this would be available if else condition and corresponding code was not run previously
}
   
    


    

    
