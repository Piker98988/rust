//! Strings and String refs; Book chapter 6.1.1; Code convention 004::01::01
/*

// Fix error without adding new line
fn main() {
    let s: str = "hello, world";

    println!("Success!");
}

*/

// Fix error without adding new line
pub fn main() {
    #[allow(unused_variables)]
    let s: &str = "hello, world"; 
    // ? use &str instead of str

    println!("Success at 001!");
}