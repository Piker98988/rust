//! Slices; Book chapter 6.3.1; Code convention 004::03::01

/*

// Fix the errors, DON'T add new lines!
fn main() {
    let arr = [1, 2, 3];
    let s1: [i32] = arr[0..2];

    let s2: str = "hello, world" as str;

    println!("Success!");
}
*/


// Fix the errors, DON'T add new lines!
#[allow(unused_variables)]
pub fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world" as &str;

    println!("Success!");
}