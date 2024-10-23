//! Ref 001, Book chapter 5.2.6, Code convention 003::02::06
/*

fn main() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，don't change other code
    let __ r2 = c;

    assert_eq!(*r1, *r2);
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

*/

pub fn main() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，don't change other code
    let ref r2 = c;
    // ? Every day I like this language better. This is insane. Everything just works. Compare this to python and python becomes a little kid's playground...

    assert_eq!(*r1, *r2);
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success at 003::02::06!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    //! Part of the exercise, it gives the memory address of whatever is given, given this whatever is a reference and a char type.
    // ? Better be writing some docs just in case
    format!("{:p}", r)
}