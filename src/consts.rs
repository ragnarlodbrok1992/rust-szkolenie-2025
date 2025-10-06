use std::sync::Mutex;
use std::sync::LazyLock;

// Globals - declering outside of other scopes
// This value has 'static lifetime
static COURSE_NAME: &str = "Rust - szkolenie 2025";
static mut FAKE_COURSE_NAME: &str = "Rust - szkolenie 2025";

const POINTS_FOR_COURSE: i32 = 69;

fn did_user_pass_the_course(user_points: i32) -> bool {
    user_points > POINTS_FOR_COURSE
}

// Approach for static mutable state
static GREETING: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("Hello, world!".to_string()));

fn main() {
    // Accessing constant in main thread
    println!("Name of the course: {}", COURSE_NAME);
    println!("To pass the threshold is --> {}", POINTS_FOR_COURSE);

    let some_user_points = 68;

    println!("Have I passed? {}", did_user_pass_the_course(some_user_points));

    // Modifying constant value
    // POINTS_FOR_COURSE = 65; // TODO: FIXME! Comment out this value

    // Modifing static lifetime variables is unsafe
    // Cannot change COURSE_NAME - it would require for it to be mutable, which it is not for this
    // example code
    unsafe {
        println!("Course name? {}", FAKE_COURSE_NAME);
        FAKE_COURSE_NAME = "Golang - szkolenie 2025!";
        println!("New course name! {}", FAKE_COURSE_NAME);
    }

    // Modifing static mutable state with Mutex
    // println!("{:?}", GREETING);  // Uncomment and see uninit LazyLock - it's LazyCell - value
                                    // initialized on first access
    let mut greeting = GREETING.lock().unwrap();
    println!("{:?}", greeting);

    *greeting = "Goodbye, cruel world!".to_string();  // * dereferences the MutexGuard and we can
                                                      // acesss String underneath
    println!("{:?}", greeting);
    // Lock is released when we go out of scope
}
