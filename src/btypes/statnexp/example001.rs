pub fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    #[allow(unused_must_use)]
    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z` ok bro
        2 * x;
    };

    println!("x is {:?} at 002.03.01", x);
    println!("y is {:?} at 002.03.01", y);
    println!("z is {:?} at 002.03.01", z);
}