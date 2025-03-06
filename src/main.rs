//Calling Methods from other methods
#[derive(Debug)]
struct Events {
    what: String,
    when: u16,
    who: bool,
}

impl Events {
    fn what_event(self: &Self) {
        println!("{}", self.what);
        println!("It has been {} years since the event took place", self.years_since_event());//calling a method instead of a field
        println!("Was it the Jews? {}", self.who);
    }

    fn years_since_event(&self) -> u16 {
        2025 - self.when
    }

    //fn happened_before(&self, other: &Self) -> bool {//returns a bool value comparing the when value of self and other
       // self.when < other.when
   // }

   // fn add_text(&mut self, other: &mut Self) {//this function calls in both instances and concatenates text to the end of the when field
       // self.what.push_str(" and it gets gayer every year");
       // other.what.push_str(" was started by you know who");
       // println!("{}", self.what);
       // println!("{}", other.what);
       // println!("{:#?}, {:#?}", self, other);//prints out full struct instance
   // }
//}

//fn struct_return(what: String, when: u16, who: bool) -> Events {
  //  Events {
    //    what,
    //    when,
    //    who,
   // }
//}

fn main() {
    let event = Events {
        what: String::from("The West turned fake and gay"),
        when: 1971,
        who: true,
    };

    event.what_event();


   // let mut event_two = Events {
      //  what: String::from("World War II"),
      //  when: 1939,
      //  who: true,
   // };

   // event.what_event();

   // match event.happened_before(&event_two) {//called happened_before on event variable. This value is passed in automatically for self, &event_two passed in for other parameter
    //   true => println!("{} happened before {}", event.what, event_two.what),
    //    false => println!("{} happened before {}", event_two.what, event.what),
  //  };

  // event.add_text(&mut event_two);

  // let bitcoin = struct_return(String::from("The West Is Saved"), 2009, true);
  // println!("Because of Bitcoin, in {}, {}? {}", bitcoin.when, bitcoin.what, bitcoin.who);
}
