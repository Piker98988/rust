//! Ownership 004, Book chapter 5.1.4, Code convention 003::01::04
/*
// Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}
*/

// Fix the error without removing any code
pub fn main() {
    let s = String::from("Hello World");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}
