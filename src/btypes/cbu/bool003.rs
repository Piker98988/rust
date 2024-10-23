/*
// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    if !t {
        println!("Success!");
    }
} 
*/

// Make println! work
pub fn main() {
    let f: bool = false;

    #[allow(unused_variables)]
    let t = true;
    if !f { // used f instead of t
        println!("Success at 002.02.03!");
    }
} 