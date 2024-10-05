//! Ownership 001, Book chapter 5.1.1, Code convention 003::01::01
// ! Going to write docs from now on. It was getting a bit messy. Changed the mod.rs files to include the double slash bang for documentation.

/*
fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x;
    println!("{}, {}",x, y);
}
*/


pub fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("Success at 003.01.01!");
    // // let y = x;
    // ? Now, you would say that the above code is wrong, but you'd be incorrect. That code would be usable as long as you do the borrowing from the start, be it, you place the & character right before `String::from("")`.
    
    #[allow(unused_variables, reason="Unreadable in editor")]
    let y = &x; // * This is my solution. The most elegant. (I'm the best)
    #[allow(unused_variables, reason="Unreadable in editor")]
    let y = x.clone();

    #[allow(unused_variables, reason="Unreadable in editor")]
    let y = x.as_str();

    let x = "Success at 003.01.01!"; 
    let y = x;
    // ? Explanation: Here, the variable x is no longer a String type, but a &str, which are different and have different behaviors. Also, &str is said "String slice".

    println!("{}, {}",x, y);
}
