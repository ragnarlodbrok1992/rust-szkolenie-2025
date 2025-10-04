use std::any::type_name;

// To suppres some warnings, since this is just example code for types
#[allow(unused_variables)]
fn main() {
    println!("Primitive types in Rust.");
    
    // Rust data Types - primitive ones
    // Statically typed language!
    // let x: i32 = "Hello!"; // Uncomment for compile-time error
    // let y: str = 42;       // Uncomment for compile-time error

    let x = 1234;

    let y: i32 = 1234;
    let z: u32 = 1234;

    // let a: u8 = 1234; // Uncomment for compile-time error

    print_type(&x);
    print_type(&y);
    print_type(&z);
    
    // Rust data Types - strings (łańcuchy znaków)
    // let y: str = "Hello!"; // Uncomment for compile-time error - str vs &str
    let y: &str = "Hello!";

    // Example with String
}

// Helper functions
fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}
