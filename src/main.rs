//Methods on a Struct
#[derive(Debug)]
struct Events {
    what: String,
    when: u16,
    who: bool,
}

impl Events {
    fn what_event(self: &Self) {//ownership is not taken, can also be written (&self)
        println!("{}", self.what);//self.field_name
        println!("The year that the wheels fell off? {}", self.when);
        println!("Has it gotten worse? {}", self.who);
    }

    fn double_when(self: &mut Self) {//ownership is not taken, but able to modify. Can also be written (&mut self)
        self.when = self.when * 2;
        println!("New date is {}", self.when);
    }
}

fn main() {
    let mut event = Events {//had to change to mut event for double_when function
        what: String::from("The West turned fake and gay"),
        when: 1971,
        who: true,
    };

    event.what_event();//the fn is called on the end of the variable. self parameter is automatically passed in. In this case, ownership is taken by the self parameter.
    event.double_when();//when called, it updates the self.when field.
    event.what_event();//date has been changed on this print out due to double_when modifying the struct instance, but what_event is still valid and usable.
}
