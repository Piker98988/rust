/*

fn main() {
    let x = 1; 
}

// Warning: unused variable: `x`

*/

#[allow(unused_variables)] // Warning supressed by telling cargo to allow it (very clever, i admit it.)
pub fn main() {
    let _x = 1; // Warning supressed with the underscore
    let y = 1;
    println!("Warnings supressed at 01.007!");
}