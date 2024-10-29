//! Rust by practice. Self hosted mdbook with exercises to master rust.

#[allow(unused_imports)]
use rust_by_practice::variables::*;
// ? Variables

#[allow(unused_imports)]
use rust_by_practice::btypes::nums::*;
// ? Numbers from basic types

#[allow(unused_imports)]
use rust_by_practice::btypes::cbu::*;
// ? Char, Bool and Unit from basic types

#[allow(unused_imports)]
use rust_by_practice::btypes::statnexp::*;
// ? Statements and expressions from basic types

#[allow(unused_imports)]
use rust_by_practice::btypes::funcs::*;
// ? Functions from basic types

#[allow(unused_imports)]
use rust_by_practice::ownnbow::own::*;
// ? Ownership from Ownership and Borrowing

#[allow(unused_imports)]
use rust_by_practice::ownnbow::refnbow::*;
// ? Reference and Borrowing from Ownership and Borrowing

#[allow(unused_imports)]
use rust_by_practice::compounds::strings::*;
// ? Strings from Compound Types

#[allow(unused_imports)]
use rust_by_practice::compounds::tuples::*;
// ? Tuples from Compound Types

#[allow(unused_imports)]
use rust_by_practice::compounds::arrays::*;
// ? Arrays from Compound Types

#[allow(unused_imports)]
use rust_by_practice::compounds::enums::*;
// ? Enums from Compound Types

#[allow(unused_imports)]
use rust_by_practice::compounds::slices::*;
// ? Slices from Compound Types

#[allow(unused_imports)]
use rust_by_practice::compounds::structs::*;
// ? Structs from Compound Types


fn main() {
    //! The main function does not do anything by itself. It just prints a guide to use this code, not anything more.
    println!("Welcome to my 'Rust by practice' notes.\nHere I write the exercises I make day after day.\nThe book can be found at https://practice.rs, if you want to follow it with me.");
    println!("To run the code, instead of doing `cargo run` you should `cargo test` instead.\nThat will run the tests which contain the code.");
    println!("To see the output of the code you should do a `cargo t -- --show-output`.");
}


#[cfg(test)]
mod tests {
    //! Each test will run all of the files inside of the module.
    //! 
    //! The naming will be comprised of:
    //! - two letters signifying the test number
    //! - the parent module
    //! - the module
    //! - the parent module number
    //! - the module number
    //! 
    //! each separated by an underscore:
    //!
    //! `fn bt_goodpractices_spaces_22_4() {...}`

    use super::*;

    #[test]
    fn aa_variables_01() {
        //! Module 001
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
        println!("Mod 001 success!")
    }

    #[test]
    fn ab_btypes_nums_02_1() {
        //! Module 002::01
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
        println!("Mod 002::01 success!")
    }

    #[test]
    fn ac_btypes_cbu_02_2() {
        //! Module 002::02
        print!("\n");
        println!("Running mod 002::02...");
        char001::main();
        char002::main();
        bool003::main();
        bool004::main();
        unit005::main();
        unit006::main();
        println!("Mod 002::02 success!")
    }

    #[test]
    fn ad_btypes_statnexp_02_3() {
        //! Module 002::03
        print!("\n");
        println!("Running mod 002::03...");
        example001::main();
        exercise002::main();
        exercise003::main();
        exercise004::main();
    }

    #[test]
    fn ae_btypes_funcs_02_4() {
        //! Module 002::04
        print!("\n");
        println!("Running mod 002::04...");
        functions001::main();
        functions002::main();
        functions003::main();
        diverge004::main();
        diverge005::main();
    }

    #[test]
    fn af_ownnbow_own_03_1() {
        //! Module 003::01
        print!("\n");
        println!("Running mod 003::01...");
        owner001::main();
        owner002::main();
        owner003::main();
        owner004::main();
        owner005::main();
        mut006::main();
        mut007::main();
        pmove008::main();
        pmove009::main();
    }

    #[test]
    fn ag_ownnbow_bow_03_2() {
        //! Module 003::02
        print!("\n");
        println!("Running mod 003::02...");
        reference001::main();
        reference002::main();
        reference003::main();
        reference004::main();
        reference005::main();
        ref006::main();
        bowrules007::main();
        mutability008::main();
        mutability009::main();
        nll010::main();
        nll011::main();
    }

    #[test]
    fn ah_compounds_strings_04_1() {
        //! Module 004::01
        print!("\n");
        println!("Running mod 004::01...");
        strnstr001::main();
        strnstr002::main();
        string003::main();
        string004::main();
        string005::main();
        string006::main();
        strnstrings007::main();
        strnstrings008::main();

        // TODO
    }
}
