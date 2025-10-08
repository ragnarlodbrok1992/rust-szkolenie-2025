fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    println!("Borrowing");
    // We'd like to access data without taking ownership over it.
    // Rust uses a borrowing mechanism. The compiler statically quarantees
    // (via its borrow checker) that references ALWAYS point to a valid objects.

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        // eat_box_i32(boxed_i32); // Uncomment

        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}
