/*

// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");
}

*/

// Remove something to make it work
#[allow(dead_code)]
pub fn main() {
    let x = 5;
    #[allow(unused_variables)]
    let mut y: u32 = 5;

    
    y = x;
    
    #[allow(unused_variables)]
    let z = 10; // Type of z ? 

    println!("Success! at 002.01.01");
}
