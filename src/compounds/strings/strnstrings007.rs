//! &str and String 001; Book chapter 6.1.7; Code convention 004::01::07
/*
// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s)
}

fn greetings(s: String) {
    println!("{}", s)
}
*/

// Fix error with at least two solutions
pub fn main() {
    let s = String::from("hello, world");
    greetings(s)
    // ? Solution 1: Make s a String type instead of &str
    // ? Solution 2: Make greetings() use a &str type instead of String
}

fn greetings(s: String) {
    println!("{}", s)
}