/*
// Make it work
fn main() {
    let c1 = "中";
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}
*/

// Make it work
pub fn main() {
    let c1: char = '中'; // Remember for next time that chars only with single quotes!! ''''''
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{} at 002.02.02", c);
}