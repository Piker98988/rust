//! String Slices; Book chapter 6.3.4; Code convention 004::03::04
/*

fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..2];

    assert!(slice == "你");

    println!("Success!");
}
*/


pub fn main() {
    let s = "你好，世界"; // ? each one spans 3 bytes
    // Modify this line to make the code work
    let slice = &s[..3]; // ? So the start of the second char is at index 3

    assert!(slice == "你");

    println!("Success!");
}