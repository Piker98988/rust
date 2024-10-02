/*
fn main() {
   let v = (let x = 3);

   assert!(v == 3);

   println!("Success!");
}
*/

pub fn main() {
    let v = {let x = 3; x}; // Wtf was that mess?
 
    assert!(v == 3);
 
    println!("Success at 002.03.03!");
 }