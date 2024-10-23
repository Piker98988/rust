/*

// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

*/

pub fn main() {
    #[allow(unused_assignments)]
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    #[allow(unused_variables)]
    let x = x;

    #[allow(unused_variables)]
    let y = 4;
    // Shadowing
    #[allow(unused_variables)]
    let y = "I can also be bound to text!";

    println!("Success at 01.006!");
}