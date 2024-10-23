//! Reference 002, Book chapter 5.2.2, Code convention 003::02::02
/*

fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, y);

    println!("Success!");
}

*/


pub fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(&5, y);
    // ? ez

    println!("Success at 003::02::02!");
}