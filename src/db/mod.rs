// Import Mutex from standard library for thread-safe data access
use std::sync::Mutex;

// Create a static mutable database that holds a vector of static strings
// Mutex ensures thread-safe access to the shared data
static DATABASE: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

// Asynchronous function to insert a value into the database
// Takes a static string reference as input
pub async fn insert(value: &'static str) {
    // Try to acquire the lock on the database
    if let Ok(mut db) = DATABASE.lock() {
        db.push(value);
        println!("{:?}", *db);
        println!("ok");
    }
}

// Asynchronous function to retrieve all data from the database
// Returns a clone of the vector containing all stored strings
pub async fn get_data() -> Vec<&'static str> {
    // Acquire the lock, unwrap the result, and clone the vector
    DATABASE.lock().unwrap().clone()
}
