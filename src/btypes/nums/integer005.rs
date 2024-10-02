/*

// Fix errors and panics to make it work
fn main() {
    let v1 = 251_u8 + 8;
    let v2 = i8::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }

 */


// Fix errors and panics to make it work
pub fn main() {
    let v1: u16 = 251 + 8;
    let v2: u16 = u16::checked_add(251, 8).unwrap();
    println!("{},{} at 002.01.05",v1,v2);
 }