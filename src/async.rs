use tokio::time::{sleep, Duration};

async fn say_hello_after(delay: u64, message: &str) {
    sleep(Duration::from_secs(delay)).await;
    println!("{}", message);
}

#[tokio::main]
async fn main() {
    let task1 = say_hello_after(2, "Hello from task 1!");
    let task2 = say_hello_after(4, "Hello from task 2!");
    let task3 = say_hello_after(3, "Hello from task 3!");
    let task4 = say_hello_after(6, "Hello from task 4!");
    let task5 = say_hello_after(5, "Hello from task 5!");

    // Run all tasks concurrently
    tokio::join!(task1, task2, task3, task4, task5);
}
