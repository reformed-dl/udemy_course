//Associated functions, constructors
//Associated functions are functions attached to a type, example String::from() and String::new(), these are functions attached to the String type, not methods.
//Often use associated function for constructors. Constructor is a function that returns a new instance of the type
//will always be fn new()
//written inside of the impl block, but Rust can differentiate between a method and an associated function. Methods will always have 'self' in the parameter
#[derive(Debug)]
struct Events {
    what: String,
    when: u16,
    who: bool,
}
impl Events {
    fn new(what: String, when: u16, who: bool) -> Self {//this will construct a new instance of the struct
        Self {
            what,
            when,
            who,
        }
    }   
    fn what_event(self: &Self) {//method to be called in main
        println!("{}", self.what);
        println!("Kicked off in {}", self.when);
        println!("Was it the Jews? {}", self.who);
    }
}

fn main() { 
    let event = Events::new(String::from("World War II Began"), 1939, true);//'::' accesses the associated function, then pass in arguments for new instance of struct
    println!("{:#?}", event);

    let event_two = Events {
        what: String::from("The West turned fake and gay"),
        when: 1971,
        who: true,
    };
    event_two.what_event();
    event.what_event();//can also call these methods on the new instance
}
