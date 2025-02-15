/*option 1 - compile the code by navigating to the src folder and then from the terminal running 'rustc main.rs'
run the executable file by './main'

option 2 - navigate to the top folder and then input 'cargo build', this will place the executable file in the target folder. './target/debug/about_me'


option 3 - you can compile and run the project by running 'cargo run'

'cargo clean' removes the executable folder, input 'cargo run' to re-create it

'cargo check' does not compile, only checks the source code for violations

changing from println! to print! does not print separate lines, it is one continuous print line */

fn main() {
    println!("My pseudonym is: reformed");
    println!("Christ is King of all of creation");    
    println!("I am learning to become a software engineer and my first language is Rust.");

}  
