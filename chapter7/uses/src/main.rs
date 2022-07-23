/*
    Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. 
    By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope
    just as though the hosting module had been defined in the crate root.

    
    Glob operator brings all public items in module into current scope.
*/ 

// use curly brackets for multple imports from same module, reducing number of use statements
use std::{cmp::Ordering, io}; 

// use as keyword for local scope renaming
use std::io::Result as IoResult;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// re-exporting by bringing into scope and making item available publically to others, usually wasteful
pub use crate::front_of_house::hosting; 

mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}

fn main() {
    println!("Hello, world!");
}

/*
    Best practice is to separate modules into multiple files. Module files can be moved into files that
    are named the same as the module name (this is how the compiler finds module files). 
    Submodules must be in folders where the folder name is the parent module name.
*/
