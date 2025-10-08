struct Sheep {
    naked: bool,
    name: &'static str,
}

// Impl - define implementations on types
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name()); // Check this out - it's a trait
                                                             // method, not a field
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Self - unknown type
trait Animal {
    fn new(name: &'static str) -> Self; // Self refers to implementor type

    // Method signatures
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaah??"
        } else {
            "baaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}


fn main() {
    println!("Traits - impls");

    let mut dolly: Sheep = Animal::new("Dolly"); // Comment
    // let mut dolly = Animal::new("Dolly"); // Uncomment
    dolly.talk();
    dolly.shear();
    dolly.talk();
}
