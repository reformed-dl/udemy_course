fn main() {
    //ranges and range iteration
    //Rut populates the Range<Type> based on the right side of the =, put to set it manually, std::ops::Range<i32>
    let month_days = 1..31; //this syntax exclusive and does not include the upper limit (1-30)
    println!("{:?}", month_days);

    let month_days = 1..=31; //this syntax is inclusive and includes the upper range (1-31)
    println!("{:?}", month_days);

    //iterate means to progress over the elements of a range, one   by one
    //using a for loop to iterate over the range
    for number in month_days {
        //this is literally reading 'for every 'number' in 'month_days' what you want to do in the {} block
        println!("{}", number); //iterates and prints out every number from 1-31
    }
    //for loop for characters
    let letters = 'b'..='f'; //range from 'b' up to and including 'f'
    for letter in letters {
        println!("{}", letter); //will iterate and print out b, c, d, e, f
    }

    let colors = ["Red", "Green", "Purple"];
    for color in colors {
        println!("{} is a great color", color); //will print out Red is a great color, Green is a great color, Purple is a great color
    }
}
