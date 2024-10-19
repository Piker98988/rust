//! Partial Moves 002, Book chapter 5.1.9, Code convention 003::01::09

/*
pub fn main() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (__, __) = __;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
*/

pub fn main() {
    let t = (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     let (s1, s2) = &t;
    // ? I was typing ref instead of the &
    // ? I'm stupid again

     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
