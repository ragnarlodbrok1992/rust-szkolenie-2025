// Defining a function of higher order that take a function, vector of intergers and returns vector
// of integers
fn apply_to_all<F>(f: F, data: Vec<i32>) -> Vec<i32>
where F: Fn(i32) -> i32, // F is type parameter - function taking i32 and returning i32
{
    data.into_iter().map(f).collect()
}

fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y // Comment to check error - move creates a closure and returns it
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];

    // let squared = apply_to_all(|x| x * x, data); // Uncomment for error
    let squared = apply_to_all(|x| x * x, data.clone()); // Trait Clone - we are duplicating value
                                                         // since we are going to use it later too
                                                         // Copy vs Clone
    println!("Squared: {:?}", squared);

    fn cube(x: i32) -> i32 {
        x * x * x
    }
    let cubed = apply_to_all(cube, data.clone());
    println!("Cubed: {:?}", cubed);

    let add_five = make_adder(5);
    let result = add_five(10);
    println!("Adding 5 to 10 equals: {}", result);
}
