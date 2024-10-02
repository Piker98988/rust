/*
// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);

    println!("Success!");
}
*/


/*
 * Bro thought it was funny to use hex numbers just in case you did not know about em
 * a hex -> 10 bin
 * b     -> 11
 * c     -> 12
 * d     -> 13
 * e     -> 14
 * f     -> 15
 * 10    -> 16
 0xff -> 15x16 + 15 = 255
 ! Hover over the literals to see the value in decimal
 */

// Modify `assert!` to make it work
pub fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success at 002.01.06!");
}
