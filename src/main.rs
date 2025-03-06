//Methods on a Struct (3) integrating match statements, returns and concatentation
#[derive(Debug)]
struct Events {
    what: String,
    when: u16,
    who: bool,
}

impl Events {
    fn what_event(self: &Self) {
        println!("{}", self.what);
        println!("The year that the wheels fell off? {}", self.when);
        println!("Has it gotten worse? {}", self.who);
    }

    fn happened_before(&self, other: &Self) -> bool {//returns a bool value comparing the when value of self and other
        self.when < other.when
    }

    fn add_text(&mut self, other: &mut Self) {//this function calls in both instances and concatenates text to the end of the when field
        self.what.push_str(" and it gets gayer every year");
        other.what.push_str(" was not the last world war");
        println!("{}", self.what);
        println!("{}", other.what);
        println!("{:#?}, {:#?}", self, other);//prints out full struct instance
    }
}

fn struct_return(what: String, when: u16, who: bool) -> Events {
    Events {
        what,
        when,
        who,
    }
}

fn main() {
    let mut event = Events {
        what: String::from("The West turned fake and gay"),
        when: 1971,
        who: true,
    };

    let mut event_two = Events {
        what: String::from("World War II"),
        when: 1939,
        who: true,
    };

    event.what_event();

    match event.happened_before(&event_two) {//called happened_before on event variable. This value is passed in automatically for self, &event_two passed in for other parameter
        true => println!("{} happened before {}", event.what, event_two.what),
        false => println!("{} happened before {}", event_two.what, event.what),
    };

   event.add_text(&mut event_two);

   let bitcoin = struct_return(String::from("The West Is Saved"), 2012, true);
   println!("Because of Bitcoin, in {}, {}? {}", bitcoin.when, bitcoin.what, bitcoin.who);
}
