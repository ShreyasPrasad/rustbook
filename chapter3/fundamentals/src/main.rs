/*
   Below is some nice snippets of key language constructs in Rust
*/

fn variables_and_mutability(){
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // cannot reassign to const
    
    let x = 5;
    let x = x + 1; // shadowing by using let

    /*
        A difference between mut and shadowing is that because we’re effectively 
        creating a new variable when we use the let keyword again, we can change 
        the type of the value but reuse the same name.
    */

    { // inner scope created with curly brackets 
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn data_types(){
    //Every value in Rust is of a certain data type
    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32
    // arithmetic works normally
    let floored = 2 / 3; // Results in 0
    let remainder = 43 % 5; // Results in 3
    let f: bool = false; // with explicit type annotation
    let z: char = 'ℤ'; // with explicit type annotation
    /*
        Rust’s char type is four bytes in size and represents a Unicode Scalar Value, 
        which means it can represent a lot more than just ASCII. Accented letters; 
        Chinese, Japanese, and Korean characters; emoji.
    */

    // Tuples: Fixed length group of values with a variety of types
    let tup: (i32, f64, u8) = (500, 6.4, 1); 
    let (x, y, z) = tup; // called destructuring, like in JS
    
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    //Arrays: Fixed length group of values with the same type
    let a = [1, 2, 3, 4, 5];
    let a = [3; 5]; // initializes array with 5 elements, each with value 3
    let first = a[0];
    // Runtime panic occurs for out of bounds access
}

fn functions(value: i32, unit_label: char) -> i32 {
    let y = {
        let x = 3;
        x + 1 // not a statement, is an expression, so it returns a value which is assigned to y
    };
    println!("The value of y is: {y}");
    6 // this function returns 6. We can also use return to return from the function early.
}

fn comments(){
    // straightforward
}

fn control_flow(){
    let number = 3;

    if number < 5 { // standard
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // since if is an expression, can be used on RHS of let

    // looping using loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // value after break returned and assigned to result
        }
    };

    // looping using while
    while counter == 0 {
        // standard
    }

    // looping using for/in
    let a = [10, 20, 30, 40, 50];
    for element in a { // like javascript
        println!("the value is: {element}");
    }

    //looping using for/rev
    for number in (1..4).rev() {
        println!("{number}!");
    }
}

fn main() {
    variables_and_mutability();
    data_types();
    functions(32, 'x');
    comments();
    control_flow();
}
