fn create_box() {
    let _box1 = Box::new(123i32);
}

fn main() {
    // Use this to check on memory after building target --> valgrind --leak-check=full --show-leak-kinds=all -s target/debug/raii
    println!("RAII - Resource Aquisition Is Initialization");

    let _box2 = Box::new(3232i32);

    {
        let _box3 = Box::new(2137.0f32);
    }

    for _ in 0u32..1_000 {
        create_box();
    }
}

