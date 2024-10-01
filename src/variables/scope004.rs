/*

// Fix the error with the use of define_x
fn main() {
    println!("{}, world", x); 
}

fn define_x() {
    let x = "hello";
}

*/


pub fn main () { 
    let x = define_x();
    println!("{}, world at 01.004", x);
}

fn define_x() -> String {
    let x = "hello";
    x.to_string()
}
