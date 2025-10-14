fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {

    match age() {
        0 => println!("Masz zero lat"),
        n @ 1 ..=17 => println!("Nie możesz pić piwa! masz {:?} lat", n),
        n @ 18 ..=u32::MAX => println!("Możesz pić piwo, masz już {:?} lat", n),
    }

    match some_number() {
        Some(n @ 42) => println!("The answer is: {}!", n),
        Some(n) => println!("Nudne... {}", n),
        _ => (),
    }

    // if let
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
    
    println!("Next case:");

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    println!("Next case:");

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // while let
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("More than 9, let's quit!");
            optional = None;
        } else {
            println!("i is {:?}, try again.", i);
            optional = Some(i + 1);
        }
    }
}
