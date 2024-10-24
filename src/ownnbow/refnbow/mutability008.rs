//! Mutability 001; Book chapter 5.2.8; Code convention 003::02::08

/*

fn main() {
    // Fix error by modifying this line
    let  s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}

*/

pub fn main() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");
    // ? ez

    borrow_object(&mut s);

    println!("Success!");
}

#[allow(unused_variables)]
fn borrow_object(s: &mut String) {}
