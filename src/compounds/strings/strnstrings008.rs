//! &str and String 002; Book chapter 6.1.8; Code convention 004::01::08
/*
// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = s;

    println!("Success!");
}
*/

// Use two approaches to fix the error and without adding a new line
pub fn main() {
    let s = "hello, world";
    let s1 = s;
    // ? Solution 1: remove type in s1 to get String in both
    // ? Solution 2: remove to_string() in s to get &str in both

    println!("Success!");
}