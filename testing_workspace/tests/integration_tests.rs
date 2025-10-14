// This is for testing.rs - 'testing' binary
use testing::{create_user, validate_user, process_user, filter_users, summarize_users, update_user_age, User};

#[test]
fn test_create_and_validate_user() {
    let user = create_user(1, "Alice".to_string(), 30);
    assert!(validate_user(&user));
}

#[test]
fn test_process_user() {
    let user = create_user(1, "Alice".to_string(), 30);
    let processed_user = process_user(user);
    assert_eq!(processed_user.name, "ALICE");
}

#[test]
fn test_filter_users() {
    let users = vec![
        create_user(1, "Alice".to_string(), 30),
        create_user(2, "Bob".to_string(), 25),
        create_user(3, "Charlie".to_string(), 35),
    ];
    let filtered_users = filter_users(users, 30);
    assert_eq!(filtered_users.len(), 2);
    assert_eq!(filtered_users[0].name, "Alice");
    assert_eq!(filtered_users[1].name, "Charlie");
}

#[test]
fn test_summarize_users() {
    let users = vec![
        create_user(1, "Alice".to_string(), 30),
        create_user(2, "Bob".to_string(), 25),
        create_user(3, "Charlie".to_string(), 35),
    ];
    let average_age = summarize_users(&users);
    assert_eq!(average_age, 30.0); // (30 + 25 + 35) / 3 = 30
}

#[test]
fn test_update_user_age() {
    let user = create_user(1, "Alice".to_string(), 30);
    let updated_user = update_user_age(user, 31);
    assert_eq!(updated_user.age, 31);
}

#[test]
fn test_integration() {
    // Create users
    let users = vec![
        create_user(1, "Alice".to_string(), 30),
        create_user(2, "Bob".to_string(), 25),
        create_user(3, "Charlie".to_string(), 35),
    ];

    // Filter users
    let filtered_users = filter_users(users, 30);

    // Process users
    let processed_users: Vec<User> = filtered_users.into_iter().map(process_user).collect();

    // Summarize users
    let average_age = summarize_users(&processed_users);

    // Validate the results
    assert_eq!(processed_users.len(), 2);
    assert_eq!(processed_users[0].name, "ALICE");
    assert_eq!(processed_users[1].name, "CHARLIE");
    assert_eq!(average_age, 32.5); // (30 + 35) / 2 = 32.5
}
