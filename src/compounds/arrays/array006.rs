//! Arrays; Book chapter 6.2.6; Code convention 004::02::06

/*

// Fix the error
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[2];

    println!("Success!");
}

*/

// Fix the error
#[allow(unused_variables)]
pub fn main() {
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0: &String = names.get(0).unwrap();
    // ? Fucking safe even. GOAT language

    // But indexing is not safe
    let _name1: &String = &names[1];
    // ? As expected...

    println!("Success!");
}
