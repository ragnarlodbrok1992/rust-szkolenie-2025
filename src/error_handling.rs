fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

// fn drink(beverage: &str) {
//     if beverage == "lemonade" { panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAA!"); }
// 
//     println!("Refreshing {}", beverage);
// }

// Second version for unwind and abort
#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party, run!");
}

// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         ah();
//     } else {
//         println!("Refreshing {}", beverage);
//     }
// }

// Third version - for Option & unwrap
fn drink(drink: Option<&str>) {
    let inside = drink.unwrap(); // Unwrap returns a panic when it receives a 'None'
    if inside == "lemonade" { panic!("AAAAAAaa!!!"); }

    println!("I love {}s!!!", inside);
}

// Unpacking Option
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn calculate_div(a: i32, b: i32) -> Option<i32> {
    let result = divide(a, b)?;
    Some(result * 2)
}

// Result type functions
fn divide_result(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn check_result(result: Result<f64, String>) {
    if result.is_ok() {
        println!("Result: {}", result.unwrap());
    } else {
        println!("Error: {}", result.unwrap_err());
    }
}


fn main() {
    println!("Error handling in Rust");

    // // panic
    // drink("water");
    // drink("lemonade"); // Comment to proceed
    // drink("kopi luwak");

    // Second version - try with using '--profile panic_abort' and '--profile panic_unwind' options
    // Option & unwrap
    
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    // let nothing = None;

    drink(coffee);
    // drink(nothing);

    // Unpacking option with '?'
    let res = calculate_div(10, 2);
    println!("{:?}", res); // Output: Some(10)

    let res = calculate_div(10, 0);
    println!("{:?}", res); // Output: None

    // Result type
    let result = divide_result(10.0, 2.0);
    let result_div_by_zero = divide_result(12.0, 0.0);

    check_result(result);
    check_result(result_div_by_zero);
}
