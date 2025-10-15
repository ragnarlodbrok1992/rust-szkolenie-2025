use std::thread;

fn main() {
    // Create a vector to hold the thread handles
    let handles: Vec<_> = (0..5).map(|i| {thread::spawn(move || {
        println!("Hello from thread {}!", i);
        })
    }).collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("All threads completed!");
}
