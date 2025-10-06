// Enums
#[allow(dead_code)] // Comment and see warnings
enum EveryOperationWeCanDoOnOurWebsite {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64},
}

// Match keyword
fn event_inspector(operation: EveryOperationWeCanDoOnOurWebsite) {
    match operation {
        EveryOperationWeCanDoOnOurWebsite::PageLoad => {
            println!("Our awesome page...");
            println!("...is loading!");
        }
        EveryOperationWeCanDoOnOurWebsite::PageUnload => println!("Goodbye, You'll be missed ;(."),
        EveryOperationWeCanDoOnOurWebsite::KeyPress(c) => println!("You pressed... {}", c),
        EveryOperationWeCanDoOnOurWebsite::Paste(msg) => println!("The message is {:?}", msg),
        EveryOperationWeCanDoOnOurWebsite::Click { x, y } => {
            println!("Clicked on {} and {} coords!", x, y);
        }, // Comment one of the match
           // cases to check what
           // compiler says
    }
}

// C-like enums - fields with set discriminators
enum Number {
    Zero,
    One,
    Two,
    Three,
}

enum Color {
    RED = 0xFF0000,
    GREEN = 0x00FF00,
    BLUE = 0x0000FF,
}

fn main() {
    let pressed = EveryOperationWeCanDoOnOurWebsite::KeyPress('x');
    let entered_the_page = EveryOperationWeCanDoOnOurWebsite::PageLoad;

    // Type aliasing
    type Action = EveryOperationWeCanDoOnOurWebsite;

    let click = Action::Click { x: 100, y: 30 };

    event_inspector(entered_the_page);
    event_inspector(pressed);
    event_inspector(click);

    // Use keyword
    use crate::EveryOperationWeCanDoOnOurWebsite::{PageUnload, Paste};
    let exiting_the_page = PageUnload;
    let passing_msg = Paste("GET /home.html HTTP/1.1\r\nHost: folwark.adamczysze.org".to_owned()); // Going
                                                                                                   // from
                                                                                                   // borrowed
                                                                                                   // to
                                                                                                   // owned
                                                                                                   // (&str)
                                                                                                   // to
                                                                                                   // String

    event_inspector(passing_msg);
    event_inspector(exiting_the_page);
    println!();

    // C-like enums examples
    println!("zero is {}", Number::Zero as u8);
    // println!("three is {}", Number::Three as f32); // Uncomment for error - cast through integer
                                                      // first
    println!("three is {}", Number::Three as u128);

    println!("roses are #{:06x}", Color::RED as i32);
    println!("violets are #{:06x}", Color::BLUE as i32);
}
