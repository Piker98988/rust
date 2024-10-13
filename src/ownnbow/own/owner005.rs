//! Ownership 005, Book chapter 5.1.5, Code convention 003::01::05

/*
// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}
*/

// Don't use clone ,use copy instead
pub fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = &x;
    println!("{:?}, {:?}", x, y);
}