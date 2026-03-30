// Challenge: The following code has 3 borrow checker errors. Identify each one and fix them without using .clone():
fn main() {
    let mut names = vec!["Alice".to_string(), "Bob".to_string()];
    let first = &names[0];
    names.push("Charlie".to_string());
    println!("First: {first}");

    let greeting = make_greeting(names[0]);
    println!("{greeting}");
}

fn make_greeting(name: String) -> String {
    format!("Hello, {name}!")
}
// Solution
// Errors fixed:

//     Immutable borrow + mutation: first borrows names, then push mutates it. Fix: use first before pushing.
//     Move out of Vec: names[0] tries to move a String out of Vec (not allowed). Fix: borrow with &names[0].
//     Function takes ownership: make_greeting(String) consumes the value. Fix: take &str instead.

// fn main() {
//     let mut names = vec!["Alice".to_string(), "Bob".to_string()];
//     let first = &names[0];
//     println!("First: {first}"); // Use borrow BEFORE mutating
//     names.push("Charlie".to_string()); // Now safe — no live immutable borrow

//     let greeting = make_greeting(&names[0]); // Pass reference, not owned
//     println!("{greeting}");
// }

// fn make_greeting(name: &str) -> String {
//     // Accept &str, not String
//     format!("Hello, {name}!")
// }
