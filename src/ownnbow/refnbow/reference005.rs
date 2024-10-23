//! Reference 005, Book chapter 5.2.5, Code convention 003::02::05
/*

fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = __;
    
    p.push_str("world");

    println!("Success!");
}

*/

pub fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;
    
    p.push_str("world");

    println!("Success at 003::02::05! {:?}", p);
}