//! String 004; Book chapter 6.1.6; Code convention 004::01::06
/*
// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}
*/

// Fix errors without removing any line
pub fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}