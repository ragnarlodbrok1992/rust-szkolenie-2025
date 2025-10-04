use std::any::type_name;
use std::mem::size_of_val;

// To suppres some warnings, since this is just example code for types
#[allow(unused_variables)]
fn main() {
    println!("Primitive types in Rust.");
    println!();
    
    // Rust data Types - primitive ones
    // Statically typed language!
    println!("Information about types -->");

    let x = 1234;

    let y: i32 = 1234;
    let z: u32 = 1234;

    // let x: i32 = "Hello!"; // Uncomment for compile-time error
    // let y: str = 42;       // Uncomment for compile-time error
    // let a: u8 = 1234; // Uncomment for compile-time error

    print_type(&x);
    print_type(&y);
    print_type(&z);

    // Scalar types
    // Infering type defaults to i32 and f64
    // Integer types (signed, unsigned)
    let aa: u8 = 127;
    let ab: u16 = 12345;
    let ac: u32 = 4294967295;
    let ad: u64 = 9108230918209381;
    let af: u128 = 102391823;

    let ba: i8 = -1;
    let bb: i16 = -127;
    let bc: i32 = 12345;
    let bd: i64 = 12;
    let bf: i128 = 13;

    // Floating-point types
    let fa: f32 = 1.01234;
    let fb: f64 = 1.123e12;

    println!("f32: {}", fa);
    println!("f64: {}", fb);

    println!();
    // Representing floating point numbers in binary
    println!("Watch out for floating point numbers representation -->");
    let mut robot_check = 0.1 + 0.2;
    println!("robot_check: {}", robot_check);

    robot_check = 0.1;
    println!("robot_check: {}", robot_check);
    println!("robot_check: {:.20}", robot_check);
    println!();

    robot_check = 0.2;
    println!("robot_check: {}", robot_check);
    println!("robot_check: {:.20}", robot_check);
    println!();

    robot_check = 0.3;
    println!("robot_check: {}", robot_check);
    println!("robot_check: {:.20}", robot_check);
    println!();

    // Char types
    println!("Char type -->");
    let znak = 'a';
    let polski_znak = 'Ł';
    let nieskończoność = '∞';

    print_type(&znak);
    print_type(&polski_znak);
    print_type(&nieskończoność);

    println!("{} size --> {}", znak, size_of_val(&znak));
    println!("{} size --> {}", polski_znak, size_of_val(&polski_znak));
    println!("{} size --> {}", nieskończoność, size_of_val(&nieskończoność));

    println!();
    // Bool types
    println!("Bool type -->");
    let yes = true;
    let no = false;

    println!("{}", yes);
    println!("{}", no);

    print_type(&yes);
    print_type(&no);

    println!();
    // Unit type
    println!("Unit type -->");
    let unit = ();

    println!("Printed value of unit_type: {:?}", unit);
    print_type(&unit);

    println!();
}

// Helper functions
fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}
