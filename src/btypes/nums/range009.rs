/*
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}
*/

pub fn main() {
    //! The ranges do not take the value on the left, while they do take the one on the right
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    //* Here it takes the left value, be it x >= 97 and does not take the other value, be it x < 123
    //* Be the range for 97..123 : 97 =< x < 123
    for i in 97..123 {
        println!("{} at 002.01.09",i);
    }
}
