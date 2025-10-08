fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }

}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// 
// }

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

fn main() {
    println!("Lifetimes - example.");

    let i = 3;
    {
        let borrow1 = &i;

        println!("borrow1: {}", borrow1);
    }

    {
        let borrow2 = &i;

        println!("borrow2: {}", borrow2);

    }
    // foo<'a>
    // foo<'a, 'b> <-- lifetime of foo cannot exceed that of either 'a or 'b

    let string1 = String::from("long string is long and longer even");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }

    // println!("The longest string is {result}"); // Uncomment

}

