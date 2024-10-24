//! Mutability 002; Book chapter 5.2.9; Code convention 003::02::09

/*

// This code has no errors!
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}

*/

// ! Checking the solution cause I have no idea what to do here.
// ? The solution was not doing anything. Nice!

//
// * The solution was to understand that when borrowing a mutable object you can
// * borrow it as immutable, while borrowing an immutable object as mutable is
// * not allowed
//

// This code has no errors!
pub fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");

    println!("Success!");
}

#[allow(unused_variables)]
fn borrow_object(s: &String) {}