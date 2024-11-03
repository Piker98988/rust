//! Arrays; Book chapter 6.2.3; Code convention 004::02::03

/*
fn main() {
    // Fill the blank
    let list: [i32; 100] = __ ;

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
*/

// ! Checked solution
pub fn main() {
    // Fill the blank
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1); // ? Dynamic like python
    assert!(list.len() == 100);

    println!("Success!");
}