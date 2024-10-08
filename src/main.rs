//! Rust by practice. Self hosted mdbook with exercises to master rust.

use rust_by_practice::variables::*; 
// ? Variables

use rust_by_practice::btypes::nums::*; 
// ? Numbers from basic types

use rust_by_practice::btypes::cbu::*; 
// ? Char, Bool and Unit from basic types

use rust_by_practice::btypes::statnexp::*; 
// ? Statements and expressions from basic types

use rust_by_practice::btypes::funcs::*; 
// ? Functions from basic types

use rust_by_practice::ownnbow::own::*;
// ? Ownership from Ownership and Borrowing

fn main() {
    println!("Running mod 001...");
    binding001::main();
    binding002::main();
    scope003::main();
    scope004::main();
    shadowing005::main();
    shadowing006::main();
    unused007::main();
    destructure008::main();
    destructure_assign009::main();

    // * 001 ends here


    print!("\n");
    println!("Running mod 002::01...");
    integer001::main();
    integer002::main();
    integer003::main();
    integer004::main();
    integer005::main();
    integer006::main();
    float007::main();
    float008::main();
    range009::main();
    range010::main();
    computation011::main();

    // * 002::01 ends here


    print!("\n");
    println!("Running mod 002::02...");
    char001::main();
    char002::main();
    bool003::main();
    bool004::main();
    unit005::main();
    unit006::main();

    // * 002::02 ends here


    print!("\n");
    println!("Running mod 002::03...");
    example001::main();
    exercise002::main();
    exercise003::main();
    exercise004::main();

    // * 002::03 ends here


    print!("\n");
    println!("Running mod 002::04...");
    functions001::main();
    functions002::main();
    functions003::main();
    diverge004::main();
    diverge005::main();

    // * 002::04 ends here

    print!("\n");
    println!("Running mod 003::01...");
    owner001::main();
    owner002::main();
    // TODO

    // * 003::01 ends here


}

//TODO Make them tests instead of running in bulk!
