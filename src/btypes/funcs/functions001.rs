/*
fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x, y: i32) {
    x + y;
}
*/


pub fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y); // ! Here the sum is returning unit because of the semicolon down there

    assert_eq!(s, 3);

    println!("Success at 002.04.01!");
}

fn sum(x: i32 /* This type wasn't specified */, y: i32) -> i32 {
    // ! Errors found: due to the semicolon it is returning unit.
    // ! No type specified for x
    x + y/* <-- There was a semicolon here, so it didn't return a val */
}