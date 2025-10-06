use std::any::type_name;
use std::mem::size_of_val;

fn main() {
    // Compound types
    println!("Compound types -->");
    // Tuples
    let some_tuple_1 = (123, 3.0, "Franek");
    let some_tuple_2 = (("Zenek", true), ("Marian", false));

    println!("Some tuple nr 1: {:?}", some_tuple_1);
    println!("Some tuple nr 2: {:?}", some_tuple_2);

    print_type(&some_tuple_1);
    print_type(&some_tuple_2);
    
    println!();
    // Arrays and slices

    let tablica: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
    print_type(&tablica);
    println!("{:?}", tablica);
    
    println!("{}", tablica[2]);
    println!("{}", tablica[3]);

    // println!("{:X}", tablica); // This requires us to implement trait UpperHex
    print!("0x");
    for val in tablica.iter() {
        print!("{:X}", val);
    }
    println!();

    let zeros_array: [u8; 256] = [0; 256];
    println!("{:?}", zeros_array);
    println!("Size of zeros_array: {}", size_of_val(&zeros_array));
    println!("Length of zeros_array: {}", zeros_array.len());

    let buff_zeros: [u128; 4] = [0; 4];
    println!("{:?}", buff_zeros);
    println!("Size of zeros_array: {}", size_of_val(&buff_zeros));
    println!("Length of zeros_array: {}", buff_zeros.len());

    // Slice - using array slice to do some changes
    let babe = &tablica[2..4]; // Look at indexing - last index is exclusive (like in for loop i <
                               // max_index + 1)
    print!("0x");
    for val in babe.iter() {
        print!("{:X}", val);
    }
    println!();

    // Empty slice
    // TODO: examples of using empty slices - parser of some expressions
    let empty_array: [f32; 0] = [];
    assert_eq!(&empty_array, &[]);
    println!("{:?}", empty_array);
    
    println!();

    // Panic example - also RUST_BACKTRACE
    let x = vec![1, 2, 3];
    let _ = &x[4..4];
}

// Helper functions
fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}
