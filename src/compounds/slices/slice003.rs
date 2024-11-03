//! Slices; Book chapter 6.3.3; Code convention 004::03::03
/*

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: __ = __;
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}
*/


pub fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..=3];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}