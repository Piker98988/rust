//! Ownership 003, Book chapter 5.1.3, Code convention 003::01::03

/*
 * The code given in the exercises is the following, always the one in the big comments
 * The solved code will be below the given in the exercise
 * When I need to check solutions I will simply comment it with a !
 * The highlighted comment will explain what parts of the solution I checked
*/

/*
fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.into_bytes();
    s
}
*/


pub fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.clone().into_bytes(); 
    // ? That was fucking easy
    s
}
