//! Reference 001, Book chapter 5.2.1, Code convention 003::02::01

/*

fn main() {
   let x = 5;
   // Fill the blank
   let p = __;

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}
*/

pub fn main() {
    let x = 5;
    // Fill the blank
    let p = &x;
    // ! Had to get the solution here. Don't know how to get the memory address...
    // ? Update: checked the solutions and mine was correct. It seems I'm not that stupid.
 
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
    // ? It seems that when printing with the : it gives de address of a reference!!
 }
 