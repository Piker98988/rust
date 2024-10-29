//! String 002; Book chapter 6.1.4; Code convention 004::01::04
/*
// Fix all errors without adding newline
fn main() {
    let s = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s);
}
*/

// Fix all errors without adding newline
pub fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";
    // ? Extremely fucking tedious to fix each error line by line...

    println!("{}", s);
}