//! Reference 004, Book chapter 5.2.4, Code convention 003::02::04
/*

// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

*/


// Fix error
pub fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success! {:?} ", s);
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
