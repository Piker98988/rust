/*
// Fill the blank to make it work
fn main() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "__".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
*/

// Fill the blank to make it work
pub fn main() {
    let x = 1_000.000_1; // ? -> f64
    #[allow(unused_variables)]
    let y: f32 = 0.12; // f32
    #[allow(unused_variables)]
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success at 002.01.07!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
