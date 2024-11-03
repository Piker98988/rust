//! Operate with UTF-8 Strings 001; Book chapter 6.1.12; Code convention 004::01::12
/*

fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".__ {
        println!("{}", c)
    }
}
*/

pub fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() { // ! Had to check solution
        // ? Nice method we got here
        println!("{}", c)
    }
}