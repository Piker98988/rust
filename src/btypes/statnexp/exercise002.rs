/*
// Make it work with two ways
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, 3);

   println!("Success!");
}
*/

// Make it work with two ways
pub fn main() {
    let v = {
        let mut x = 1; //? Removing this semicolon is one
        x += 2
    };
    
    //? Changing three for the unit type is another
    assert_eq!(v, /* Three was here instead of unit --> */()); 
    
    println!("Success at 002.03.02!");
}
