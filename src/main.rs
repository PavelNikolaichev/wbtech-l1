pub trait Action {
    fn say(&self) {
        println!("Action");
    }
}

pub struct Person {
    name: String
}

impl Person {
    pub fn new(name: &str) -> Person {
        Person {
            name: String::from(name)
        }
    }
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let person: Person = Person::new("Alice");
    person.say();

    let person2: Person = Person::new("Bob");
    person2.say();

    let person3: Person = Person::new("");
    person3.say();
}


