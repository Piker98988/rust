//! Reference 003, Book chapter 5.2.3, Code convention 003::02::03
/*

// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(s);

    println!("Success!");
}

fn borrow_object(s: &String) {}

*/

// Fix error
pub fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success at 003::02::03!");
}

fn borrow_object(s: &String) {}