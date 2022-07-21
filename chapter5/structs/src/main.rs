struct User {
    active: bool,
    /*
        Since we use String instead of &str, the struct owns all of its data and is 
        valid as long as the struct is valid.
    */
    username: String,
    email: String,
    sign_in_count: u64,
} // structs are similar to tuples, but allow referencing fields by name

struct Color(i32, i32, i32); // tuple structs
struct Point(i32, i32, i32);

struct AlwaysEqual; // Unit struct (no fields)

fn defining_structs(){
    // we can create instances of structs with concrete field values
    let user = User {
        active: true,
        username: String::from("test"),
        email: String::from("test@test.com"),
        sign_in_count: 4
    };
    /*
    if instance is mutavble, we can use dot notation. 
    We cannot mark only a subset of the fields as mutable, entire instance must be.
    */
    let mut instance = user;
    instance.active = false;
    println!("{}", instance.active); // it changes to false as expected

    // we can update only certain fields, depending on the set fields of other instances
    let user_2 = User {
        active: false,
        ..instance // again, similar to javascript
    }; 
    println!("{}", instance.email); //throws an error since it is Move!
    println!("{}", instance.sign_in_count); //does not throw an error since it is Copy!

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    /*
        A function that takes a parameter of type Color cannot take a Point as an argument, 
        even though both types are made up of three i32 values.
        Otherwise, struct tuples behave like normal tuples (indexing dot notation).
        No names are associated with fields.
    */

    let subject = AlwaysEqual; // Useful for define traits
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, 
        email, // shorthand similar to javascript
        sign_in_count: 1
    }
}

fn main() {
    println!("Hello, world!");
    defining_structs();
}
