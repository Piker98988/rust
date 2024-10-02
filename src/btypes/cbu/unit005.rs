/*
// Make it work, don't modify `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
*/

// Make it work, don't modify `implicitly_ret_unit` !
pub fn main() {
    let _v: () = ();

    #[allow(unused_variables)]
    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success at 002.02.05!");
}

fn implicitly_ret_unit() {
    println!("I will return a () at 002.02.05"); // unit is like void?
}

// Don't use this one
#[allow(dead_code)]
fn explicitly_ret_unit() -> () {
    println!("I will return a () at 002.02.05");
}