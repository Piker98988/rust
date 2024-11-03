//! Arrays; Book chapter 6.2.1; Code convention 004::02::01

/*
fn main() {
    // Fill the blank with proper array type
    let arr: __ = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 4);

    println!("Success!");
}
 ? Notes:
 * Arrays must have a length given, as it is part of the type signature
 * You cannot initialize an array without the number of elements given, it has to be known at compile time.
 ! Given the complexity with arrays in rust comparing it to python, I'll have to work in these some more...
*/

pub fn main() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // ? Okay, the type signature is the type of the elements in the array
    // ? and then the amount of those elements, so it means we cannot have
    // ? dynamic lists with several different data types inside.

    // Modify the code below to make it work
    assert!(arr.len() == 5);
    // ? Better than that .__len__() abomination in python...

    println!("Success!");
}