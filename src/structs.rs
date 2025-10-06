#[allow(dead_code)]  // Disallow warning for dead code
#[derive(Debug)] // Look at line 49
struct User {
    active: bool,
    username: String,
    email: String,
    age: u8,
}

#[derive(Debug)]
struct AlwaysEqual;

#[allow(dead_code)]
#[derive(Debug)]
struct Pair(i32, f32);

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    println!("Structs");

    // There are three types of structures (structurized data) in Rust
    // Tuple structs - basically named tuples
    let para = Pair(123, 21.37);

    println!("Named tuple Pair: {:?}", para);

    println!("Named tuple second field: {}", para.1);

    // Destructuring named tuple
    let Pair(integer, floating_point) = para;

    println!("Integer: {}, floating_point: {}", integer, floating_point);
    
    // Structs are somewhat tuples, but with named fields
    // Classic C structs
    let name = "Piotrek";
    let email_piotrka = "piotrek@piotrek.pl";

    let piotrek = User {
        active: false,
        username: name.to_string(), // mismatched types, string literal!
        email: email_piotrka.to_string(),
        age: 28,
    };

    println!("Piotrek: {:?}", piotrek); // Error! "type doesn't implement Debug"

    let aktywny_piotrek = User {
        active: true,
        ..piotrek
    };

    println!("Aktywny Piotrek: {:?}", aktywny_piotrek);

    let user_from_builder = build_user(String::from("Adam"), String::from("adam@adamczycha.pl"), 45);
    
    println!("New user!: {:?}", user_from_builder);

    // Unit structs - field-less, useful for generics
    let unit_struct = AlwaysEqual;

    println!("Unit struct that will be always equal: {:?}", unit_struct);

    // Defining object of type with lifetimes
    let record = Rekord {
        imie: "Mariusz",
        nazwisko: "Sanitariusz"
    };

    println!("Nowy rekord w bazie danych! {:?}", record);

    // Issue with references
    let r;
    {
        let first_name = String::from("Zenek");
        let last_name = String::from("Kowal");
        r = Rekord {
            imie: &first_name,
            nazwisko: &last_name,
        };
    }
    // 'first_name' and 'last_name' go out of scope here, but 'r' still refers them
    // println!("Jeszcze nowszy rekord! {:?}", r); // Uncomment for error
}

fn build_user(username: String, email: String, age: u8) -> User {
    User {
        active: false,
        username,
        email,
        age
    }
}

// Data ownership in structs
// We store a reference in a struct
#[derive(Debug)]
struct Rekord<'lifetime> {
    imie: &'lifetime str,
    nazwisko: &'lifetime str,
}
