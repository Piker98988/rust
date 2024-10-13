//! Mutability 001, Book chapter 5.1.6, Code convention 003::01::06
/*
// make the necessary variable mutable
fn main() {
    let s = String::from("Hello ");
    
    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}
*/


// make the necessary variable mutable
pub fn main() {
    let s = String::from("Hello ");
    
    let mut s1 = s;
    // ? Just add that mut here...

    s1.push_str("World!");

    println!("Success!");
}