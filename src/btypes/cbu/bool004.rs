/*
// Make it work
fn main() {
    let f = true;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}
*/

// Make it work
pub fn main() {
    let f = true;
    let t = true || false; // true && false is false
    assert_eq!(t, f);

    println!("Success at 002.02.04!");
}