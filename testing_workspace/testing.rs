#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u32,
}

// Creates a new user
pub fn create_user(id: u32, name: String, age: u32) -> User {
    User { id, name, age }
}

// Validates the user details
pub fn validate_user(user: &User) -> bool {
    !user.name.is_empty() && user.age > 0
}

// Processes the user data (e.g., formats the name)
pub fn process_user(user: User) -> User {
    User {
        id: user.id,
        name: user.name.to_uppercase(),
        age: user.age,
    }
}

// Filters a list of users based on age
pub fn filter_users(users: Vec<User>, min_age: u32) -> Vec<User> {
    users.into_iter().filter(|user| user.age >= min_age).collect()
}

// Summarizes the list of users (e.g., calculates average age)
pub fn summarize_users(users: &[User]) -> f64 {
    let total_age: u32 = users.iter().map(|user| user.age).sum();
    let count = users.len() as f64;
    total_age as f64 / count
}

// Updates the age of a user
pub fn update_user_age(user: User, new_age: u32) -> User {
    User {
        id: user.id,
        name: user.name,
        age: new_age,
    }
}

// Running tests: cargo test --test integration_tests

