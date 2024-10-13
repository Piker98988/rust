//! Mutability 002, Book chapter 5.1.7, Code convention 003::01::07
/*
fn main() {
    let x = Box::new(5);
    
    let ...      // update this line, don't change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}
*/


pub fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(4); // update this line, don't change other lines!
    // ? Okay, so y here is a variable that we change their value, so you have to make it mut. Also, by dereferencing it down there with the asterisk, you have to make a pointer using Box::new()

    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}
/*
 * Okay, been doing some research on that * and it seems like it does this:
 * When you have a reference, say:
 --- y = &x;
 * That means y is a pointer to a value, be it, y is not x, its an arrow pointing to x.
 * When saying y what you're doing is following the arrow to x
 * By saying *y instead what you do is that you follow the arrow with * and the value becomes *y. Be it:
 --- let x = 6;
 --- let y = &x; // Here there is a reference
 --- *y = 4; // Instead of changing y it tries to change x giving a mutability error
 --- assert_eq!(x, 6);
 * If we were to make x reference in the code mutable it should reassign the value:
 --- let x = 6;
 --- let y = &mut x; 
 --- *y = 4; <-- y acts as an i32 type here
 --- y = 4; <-- While it acts as a &i32 type here
 --- assert_eq!(x, 6);
 * By running in "evcxr" we can see that the code panics
 * And that means that you cannot change the value of a reference unless you dereference it first
 * Pretty useful and insane
 * I fucking love rust
 * Best programming language 100%
*/
