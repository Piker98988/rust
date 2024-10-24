//! NLL 002, Book chapter 5.2.11, Code convention 003::02::11
/*

fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
}

*/

#[allow(unused_variables)]
pub fn main() {
    let mut s = String::from("hello, ");


    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time

    // ? As we saw in last exercise (nll010) we should use r1 after the declaration of r2 to create an error
    // // println!("{:?}", r1)
    println!("Success at 003::02::11!")
}