//Enums and Match Statements (2)
#[derive(Debug)]

enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old operating system");//can add a block of code to print and still end with an implicit return
            39//must have this as the final line becuase all match arms must return a u32 int value
        }
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}

fn main () {
    let my_computer = OperatingSystem::Linux;
    let age = years_since_release(my_computer);
    println!("My computer's operating system is {} years old", age);

    let dads_computer = OperatingSystem::Windows;
    let dads_age = years_since_release(dads_computer);//when invoking fn, runs match logic and matches on windows which forces the execution of the block, print macro and implicit int return.
    println!("My dad's computer operating system is {} years old", dads_age);//implicit return of match arm is interpolated into the print line
}
