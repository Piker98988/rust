/*

// Fix the error below with least amount of modification
fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

*/

pub fn main() {
    let (mut x, y) = (1, 2); // Make x mutable! Only 4 characters!
    x += 2;
    // Or just shadow it with another x.
    let x = 3;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success at 01.008!");
}