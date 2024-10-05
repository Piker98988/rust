//! Ownership 002, Book chapter 5.1.2, Code convention 003::01::02
//! I FUCKING LOVE TYPE ANNOTATIONS. YES I DO. I LOVE THEM.
/*
// Don't modify code in main!
fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) {
    println!("{}", s);
}
*/

// Don't modify code in main!
pub fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(&s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: &String) -> &str {
    s as &str
}