/*

// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2 == __);

    // Integer subtraction
    assert!(1i32 - 2 == __);
    assert!(1u8 - 2 == -1); 
    
    assert!(3 * 50 == __);

    assert!(9.6 / 3.2 == 3.0); // error ! make it work

    assert!(24 % 5 == __);
    // Short-circuiting boolean logic
    assert!(true && false == __);
    assert!(true || false == __);
    assert!(!true == __);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
*/


// Fill the blanks and fix the errors
pub fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1); //? Changed type of the 1 from u8 to i8 
    
    assert!(3 * 50 == 150);

    assert!(9 / 3 == 3); // rounding error. changed type from f64 to u8

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b} at 002.01.11", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b} at 002.01.11", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b} at 002.01.11", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {} at 002.01.11", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x} at 002.01.11", 0x80u32 >> 2);
    /*
     ? The bitwise logic operations you already know. OR, AND, XOR, NOT...
     * 0011 0011 0011
     * 0101 0101 0101
     * AND- -OR- -XOR
     * 0001 0111 0110

     ? Bit shift operations (l and r)
     ! a >> b :: a will be shifted to the left b amount of times
     * 1 << 5 -> As one is the first position, it is shifted five times, be it, it will end up in the 6th position: 64
     * 0x80 >> 2 -> 0x80 has to be converted from hex to binary (spoiler: 128d) and it will end up like 0b10000000, and it will be shifted two times to the right, so it will end up like 0b00100000, which is 32 in dec
     */
}
