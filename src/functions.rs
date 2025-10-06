// Remember lifetime of static global variables
static HARRY_POTTER: &str = "Harry Potter and Philosophers Stone";

struct Book {
    title: String,
}

// Function for Book struct
fn get_title(book: &Book) -> &str {
    &book.title
}

// Function to create a string
fn create_string() -> String {
    String::from("Hello, Rust!")
}

// Function taking and returning ownership
fn take_and_return_ownership_of_title(title: String) -> String {
    println!("Today we are going to read: {}", title);
    title
}

// Function takes ownership, modifies the value, and returns ownership
fn modify_title(mut title: String, suffix: &str) -> String {
    title.push_str(suffix);
    title
}

// Functon that modifies the value AND returns it
fn process_title(title: String) -> String {
    let modified = modify_title(title, " 2: Electric Boogaloo");
    modified
}

// Function compering two string references and returns the longest one
fn longest_title<'a>(title_1: &'a str, title_2: &'a str) -> &'a str {
    if title_1.len() > title_2.len() {
        title_1
    } else {
        title_2
    }
}

fn main() {
    // Creating rust book
    let rust_title = create_string();
    println!("Rust book title: {}", rust_title);

    // Take - return ownership
    let new_rust_title = take_and_return_ownership_of_title(rust_title);
    println!("New rust title: {}", new_rust_title);

    // println!("Old rust title: {}", rust_title);  // Uncomment to see error

    // Passing ownership to more deep nested function
    let rust_title_sequel = process_title(new_rust_title);
    println!("Prepare for... {}", rust_title_sequel);

    // Example with lifetimes
    let book = Book {
        title: String::from("Ogniem i Mieczem"),
    };

    let title_ref = get_title(&book);
    println!("Nowa książka w bibliotece: {:?}", title_ref);

    let novel = String::from("Treny: Kochanowskiego");
    let longest_novel = longest_title(&novel, &book.title);
    println!("Longest title: {:?}", longest_novel);

    // Example with static value
    let novel_ref = longest_title(HARRY_POTTER, &book.title);
    println!("Longest title from static and non-static: {:?}", novel_ref);

    // Closures
    let marża_sprzedawcy = 5;
    let książka_z_ceną = |cena: i32, tytuł: String| println!("Książka {:?} kosztuje {}", tytuł, cena + marża_sprzedawcy);

    let marża_sprzedawcy = 10;

    książka_z_ceną(42, novel_ref.to_string());
}
