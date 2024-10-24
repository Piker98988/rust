//! NLL 001; Book chapter 5.2.10; Code convention 003::02::10

/*

// Comment one line to make it work
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    println!("{}",r1);
}

*/

// ! Checked solution
// Comment one line to make it work
pub fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    // ? If you don't use r1 after the declaration of r2 it is indeed valid.
    // ? Interesting.

//    println!("{}",r1);
}
