//Enums and Match Statements
//Match Statements work well with Enums as Enums are a specific set of possible variants
#[derive(Debug)]

enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

fn years_since_release(os: OperatingSystem) -> u32 {//parameter is set to the type of enum and returns a u32 int value
    match os {
        OperatingSystem::Windows => 39,//each arm represents one of the enum variants and returns a u32 int value
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}

fn main () {
    let my_computer = OperatingSystem::Linux;//set a variable equal to one of the enum variants
    let age = years_since_release(my_computer);//set a variable equal to the fn with the variable passed in as the argument (variable equals the enum variant)
    println!("My computer's operating system is {} years old", age);
}
