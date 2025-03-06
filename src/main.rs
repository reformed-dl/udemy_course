//Methods on a Struct
#[derive(Debug)]
struct Events {
    what: String,
    when: u16,
    who: bool,
}

        /*Parameter Types: 1st parameter must always be self.
        4 Options: 
        1) Immutable struct value, self parameter takes ownership (self or self: Self)
        2) Mutable struct value, self parameter takes ownership and can modify (mut self or self: mut Self)
        3) Immutable reference to a struct instance, no ownership (&self or self: &Self)
        4) Mutable reference to a struct instance, no ownership but can modify (&mut self or self: &mut Self) */

impl Events {
    fn what_event(self: Self) {
        println!("{}", self.what);
        println!("The year that the wheels fell off? {}", self.when);
        println!("Has it gotten worse? {}", self.who);
    }
}

fn main() {
    let event = Events {
        what: String::from("The West turned fake and gay"),
        when: 1971,
        who: true,
    };

    event.what_event();//the fn is called on the end of the variable. self parameter is automatically passed in. In this case, ownership is taken by the self parameter.
}
