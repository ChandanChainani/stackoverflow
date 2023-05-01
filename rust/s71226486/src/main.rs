use enum_iterator::IntoEnumIterator;

trait CheeseBoard {
    fn say_cheese (self);
}

struct Cheese {
    name: String,
}

impl CheeseBoard for Cheese {
    fn say_cheese(self) {
        println!("I am {}", self.name);
    }
}

struct Person {
    name: String,
}

impl CheeseBoard for Person {
    fn say_cheese (self) {
        println!("{} says cheese!", self.name);
    }
}

#[derive(IntoEnumIterator)]
enum CheesyPerson {
    Cheese(Cheese),
    Person(Person),
    UncheesyNonperson,
}

fn main() {
    let _a = [CheesyPerson::Cheese(Cheese { name: "Gouda".into() }), CheesyPerson::Person(Person { name: "Peer".into() }), CheesyPerson::UncheesyNonperson];
    // todo!("Call say_cheese on items in _a where the enum variant has exactly one field that implements the CheeseBoard trait.")
    for t in _a.iter() {
        for e in t.into_enum_iter() {
            e.say_cheese();
        }
    }
}
