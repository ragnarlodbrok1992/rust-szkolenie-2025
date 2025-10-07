struct Point {x: i32, y: i32, z: i32}

#[allow(overflowing_literals)]
fn main() {
    // Inference (ex. from - https://doc.rust-lang.org/rust-by-example/types/inference.html)
    let elem = 5.0f32;

    let mut vec = Vec::new();  // Empty vector (growable array)

    vec.push(elem); // Try commenting this line

    println!("{:?}", vec);

    // Shadowing
    let some_variable = 1;

    {
        println!("{}", some_variable);

        let some_variable = "abc";

        println!("{:?}", some_variable);
    }
    println!("{:?}", some_variable);

    let some_variable = 3;

    println!("{:?}", some_variable);

    // Aliasing
    // 1. Data can be immutably borrowed any number of times, but while that
    // 1a. It cannot be mutably borrowed.
    // 2. Only ONE mutable borrow is allowed at a time.
    // Original value can be borrowed again
    let mut point = Point {x: 0, y: 1, z: 2};

    let borrowed_point = &point;
    let another_borrow = &point;

    println!("Point has coordinates --> {} {} {}", borrowed_point.x, another_borrow.y, point.z);

    // let mutable_borrow = &mut point; // Uncomment

    println!("Point has coordinates --> {} {} {}", borrowed_point.x, another_borrow.y, point.z);

    let mutable_borrow = &mut point;

    // println!("Point has coordinates --> {} {} {}", borrowed_point.x, another_borrow.y, point.z);
    // Uncomment ^

    mutable_borrow.x = -1;
    mutable_borrow.y = -2;
    mutable_borrow.z = -3;

    println!("Haha mutable! {} {} {}", mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // Mutable no longer used

    let new_borrowed_point = &point;

    // Casting
    let decimal = 21.37_f32;

    // let integer: u8 = decimal; // NO IMPLICIT CASTING!
    // Uncomment ^

    let integer = decimal as u8;

    println!("Casting: {} -> {}", decimal, integer);

    println!("1000 as u16: {}", 1000 as u16);
    println!("1000 as u8:  {}", 1000 as u8); // Check "allow" attribute // Comment to see what
                                             // happens
    println!("-1 as u8:    {}", (-1i8) as u8);
}
