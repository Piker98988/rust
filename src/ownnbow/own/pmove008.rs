//! Partial Moves 001, Book chapter 5.1.8, Code convention 003::01::08

/*
fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t);
}
*/

pub fn main() {
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
    // ? I was thinking it was taking a character from the string, didn't realize it was a vector. I'm stupid.
}