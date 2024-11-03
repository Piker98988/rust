//! String index 001; Book chapter 6.1.11; Code convention 004::01::11
/*

fn main() {
    let s1 = String::from("hi,中国");
    let h = s1[0]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..5]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}
*/


pub fn main() {
    let s1 = String::from("hi,中国");
    let ref h = s1[0..1]; 
    // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // ? This is an insanity
    // Modify this line to fix the error, tips: `中` takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}
// ? Gotta love the language
