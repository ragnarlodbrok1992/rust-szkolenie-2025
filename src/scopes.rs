fn destroy_box(c: Box<i32>) {
    println!("Destorying a box that contains {}", c);
}

fn main() {
    println!("Some scopes examples");
    // Scoping rules
    // RAII --> moved to raii.rs
    // Ownership and moves

    let x = 5u32;

    let y = x;

    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);

    let b = a;

    destroy_box(b);

    // println!("b contains: {}", b); // Uncomment to see what will go on

    //   --> mutability

    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // *immutable_box = 4; // Uncomment

    let mut mutable_box = Box::new(123u32);

    *mutable_box = 2137;

    println!("mutable_box now contains {}", mutable_box);

    //   --> partial moves

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(21),
    };

    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // println!("The person struct is {:?}", person); // Uncomment

    println!("The person's age from person struct is {}", person.age);

    // Borrowing --> moved to borrow.rs
    // Lifetimes --> moved to lifetimes.rs
}
