//Practice with While Loops and For Loops
fn main () {
   outer_inner_loop(4);
   println!("The factorial of the number is {}", factorial_for(10));
   println!("The factorial of the number is {}", factiorial_while(6));
}

fn outer_inner_loop(number: i32) {//this will generate a loop, for every i iteration, there will be a full iteration of j
    for i in 1..=number {
        println!("Outside loop is {}", i);
        for j in 1..=number {
            println!("i: {}, j: {}", i, j);
        }
    }
}

fn factorial_for(number: i64) -> i64 {//this will iterate the range of 1 through the number passed in. Each loop the mutable product will * by the i, then return the new product and run again,
    let mut product = 1;//mutable product starts at 1
    for i in 1..=number {//loop will iterate through each number in the range
        product *= i;//product is mutable, starts at 1, and multiply by each number in the range, loop by loop
    }
    product//1st product is 1*1, product return is 1, then loop goes to 2 and * by product, 2*1, new product returned is 2, then 2*3, new returned product is 6, then 6*4, product returned is 24, etc.
}   

fn factiorial_while(number: i32) -> i32 {//this will run a while loop starting at the number passed
    let mut product = 1;//product starts at 1, but is mutable
    let mut count = number;//count is set at the number that will be passed in, but is mutable
    while count > 0 {//while loop will run as long as count is > 0
        product *= count;//each loop the product will * by the count. 1st loop count is 5, product is 1, result is 5
        count -= 1;//then count is decremented by one each loop
    }
    product//product is returned and loop starts over. 1st product is 5, then count drops to 4. 4*5 = 20, that is new product returned, count drops to 3*20, product becomes 60, etc.
}
