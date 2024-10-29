//! Str and &str 002; Book chapter 6.1.2; Code convention 004::01::02
/*
// Fix the error with at least two solutions
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}
*/

// Fix the error with at least two solutions
pub fn main() {
    let s: &str = "hello, world".into();
    greetings(&s);
    // ? Solution 1: borrow the value
    // ? Solution 2: Make it a string slice instead of a Box<str>
}

fn greetings(s: &str) {
    println!("{}",s)
}