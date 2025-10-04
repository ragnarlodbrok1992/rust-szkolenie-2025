use std::any::type_name;
use std::slice;

#[allow(unused_variables)]
fn main() {
    // Rust data Types - strings (łańcuchy znaków) - &str and String
    // let y: str = "Hello!"; // Uncomment for compile-time error - str vs &str
    let y: &str = "Hello!";

    let mut something = "Hello";

    print_type(&something);
    println!("Ptr: {:?}, length: {}", something.as_ptr(), something.len());
    println!("Count of characters: {}", something.chars().count());

    something = "Żółć!";
    print_type(&something);
    println!("Ptr: {:?}, length: {}", something.as_ptr(), something.len());
    println!("Count of characters: {}", something.chars().count());

    // Taken from doc.rust-lang.org
    let story = "Once upon a time..."; 

    // We can re-build a str out of ptr and len. This is all unsafe because
    // we are responsible for making sure the two components are valid:
    let s = unsafe {
        // First, we build a &[u8]...
        let slice = slice::from_raw_parts(story.as_ptr(), story.len());

        // ... and then convert that slice into a string slice
        str::from_utf8(slice)
    };
    assert_eq!(s, Ok(story));

    println!("story: {}", story);
    print_type(&story);
    println!("s: {:?}", s);
    print_type(&s);
    println!();

    // Example with String
    // Pangram - sentence containing every letter of the alphabet
    let pangram: &'static str = "pchnąć w tę łódź jeża lub ośm skrzyń fig";
    println!("Polski pangram: {}", pangram);

    let mut chars: Vec<char> = pangram.chars().collect();
    // chars.sort().dedup(); // Uncomment and check error
    chars.sort();
    chars.dedup();

    // Creating String object
    let mut polski_string = String::new();
    for c in chars {
        polski_string.push(c);
    }
    println!("Posortowane: {}", &polski_string); 

    // Alokacja na stercie
    let psiarze = String::from("Kocham pieski!");
    // Nowa alokacja ze zmienionym stringiem
    let kociarze: String = psiarze.replace("pieski", "kotki");

    println!("Psiarze mówią: {}", psiarze);
    println!("{:?}", psiarze.as_ptr());
    println!("Kociarze mówią: {}", kociarze);
    println!("{:?}", kociarze.as_ptr());
}

// Helper functions
fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}
