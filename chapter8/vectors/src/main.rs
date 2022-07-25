/*
    Collections are useful data structures provided by Rust standard lib.
    Collections are heap allocated structures that contain multiple values.
*/

// The variants of an enum are defined under the same enum type
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vectors(){
    let mut v: Vec<i32> = Vec::new(); // create empty vector, marked as mut so we can add elements
    let macro_v = vec![1, 2, 3]; // initialize vector with elements, compiler performs type inference

    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    
    match v.get(2) {
        Some(third) => {
            println!("The third element is {}", third);
        },
        None => println!("There is no third element")
    }

    // Use get method to handle an option and explicitly handle out of bounds exceptions; otherwise, use 
    // indexing if you are ok with the thread panicing.

    // push method makes a mutable borrow, so you cannot have references to elements in that vector in the same scope.

    for i in &v { // gives you immutable references to elements of the vector v
        println!("{}", i);
    }

    for i in &mut v { // gives you mutable references to elements of the vector v (this runs because for loop above goes out of scope)
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
     
} // like any struct, a vector is freed when it goes out of scope

fn main() {
    vectors();
}
