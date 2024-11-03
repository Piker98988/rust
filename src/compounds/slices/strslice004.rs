//! String Slices; Book chapter 6.3.4; Code convention 004::03::04
/*

fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[__];

    assert_eq!(slice1, slice2);

    println!("Success!");
}
*/


pub fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[0..=1];

    assert_eq!(slice1, slice2);

    println!("Success!");
}