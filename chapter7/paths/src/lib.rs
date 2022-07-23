/*  ***Paths and Crates***
    An absolute path starts from the root crate using either the crate name, or "crate" for
    the current crate.

    Relative path starts from current module, and uses "self" and "super" keywords.

    The way privacy works in Rust is that all items (functions, methods, structs, enums, 
    modules, and constants) are private by default. Items in a parent module can’t use 
    the private items inside child modules, but items in child modules can use the items in 
    their ancestor modules. 
*/

mod front_of_house { // parent module
    pub mod hosting { // child module
        pub fn add_to_waitlist() {}
    }
}

/*
    This function is part of the library crate's public API.
    The front_of_house module isn’t public, but because the eat_at_restaurant function is 
    defined in the same module as front_of_house (that is, eat_at_restaurant and front_of_house 
    are siblings), we can refer to front_of_house from eat_at_restaurant
*/
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


/* ***The super keyword***
    Use the super keyword to refer to parent module items.
*/
fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast { // public struct
        pub toast: String, // public enum field; best practice to leave other fields private
        seasonal_fruit: String,
    }

    pub enum Appetizer { // enums aren't useful unless their variants are public
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // public enum 
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

    
    

