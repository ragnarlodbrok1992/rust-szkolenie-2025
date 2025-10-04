use my_libs::sub::substract;
use my_libs::add::add;

fn main() {
    println!("Hello, world!");

    // Using library mylib
    // let result = add(34, 35); // Uncomment and run - this is example of mutability that needs to
    // be explicitly defined
    let mut result = add(34, 35);
    println!("Result of addition is: {}", result);
    result = substract(35, 34);
    println!("Result of substraction is: {}", result);
}
