//! Borrowing rules 001; Book chapter 5.2.7; Code convention 003::02::07
/*

// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

*/

// ! Checked solution here
// Remove something to make it work
// Don't remove a whole line !
pub fn main() {
    let s = String::from("hello");
    // ? Removed mut here too

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}
