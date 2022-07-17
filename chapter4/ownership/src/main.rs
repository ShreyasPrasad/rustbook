/*
    Rust manages memory through a system of ownership with a set of
    rules that the compiler checks (no inherent garbage collection, or 
    explicit memory allocation/deallocation).

    Stack vs heap allocation important. Ownership ensures that you don't
    have to think of stack vs heap often.

    Each value in Rust has an owner.
    There can only be one owner at a time. 
    When the owner goes out of scope, the value is dropped.
*/

fn what_is_ownership(){
    let g = "hello";
    /*
        g is a string literal.
        In the case of a string literal, we know the contents at compile time, 
        so the text is hardcoded directly into the final executable
    */
    // g is valid until it goes out scope.
    let s = String::from("hello");
    /*
        This type manages data allocated on the heap and as such is able 
        to store an amount of text that is unknown to us at compile time.
    
        Double colon specifies the namespace of function.
    */
    s.push_str(", world!"); // push_str() appends a literal to a String

    /*
         Memory is automatically returned once the variable that owns it goes 
         out of scope. Rust calls a special function for us. This function is 
         called drop, and it’s where the author of String can put the code to 
         return the memory. Rust calls drop automatically at the closing curly bracket.
         This is like RAII from C++.
    */

    let x = 5;
    let y = x;
    // copy of value of x created and assigned to y

    /*
        Rust has a special annotation called the Copy trait that we can place on types that 
        are stored on the stack, as integers are. 
        If a type implements the Copy trait, variables that use it do not move, but rather are 
        trivially copied, making them still valid after assignment to another variable.
    */

    let t1 = String::from("hello");
    let t2 = t1;
    // t1 and t2 point to same heap-allocated string, shallow copy, t1 is no longer valid (can't be used)

    let s1 = String::from("hello");
    let s2 = s1.clone();
    // s1 and s2 point to different strings in memory, deep copy

    /*
        Same applies to function calls and ownership. Copies work normally (original is valid).
        Passing as argument to function that doesnt implement Copy renders the initial variable invalid.
        Returning works the same way; copies work normally and non-copy returns invalidate initial variable.

        When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop 
        unless ownership of the data has been moved to another variable.
    */
}

fn references(){
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // passing reference to object as parameter

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // does not take ownership of input object
    s.len() // we are not allowed to modify the value of a reference. unless it is a mutable reference (&mut String)
    /*
        Mutable references have one big restriction: if you have a mutable reference to a value, you can have no 
        other references to that value. The benefit of having this restriction is that Rust can prevent data races 
        at compile time.

        let r1 = &s1; // no problem
        let r2 = &s1; // no problem
        let r3 = &mut s1; // BIG PROBLEM

        So at any given time, you can have either one mutable reference or any number of immutable references.
    */
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

fn slice(){
    /*
        Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. 
        A slice is a kind of reference, so it does not have ownership.
    */
    let t = String::from("hello world");

    let hello = &t[0..5];
    let world = &t[6..11];

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error, since s borrows a mutable reference to s and an immutable reference already exists (word)

    println!("the first word is: {}", word);

    let m = "string";
    /*
        The type of m is &str, which is a slice pointing to that specific point of the binary.
        This is also why string literals are immutable; &str is an immutable reference.
    */
}

// uses slicing to find the first word in a string
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    println!("Hello, world!");
    /*
        Each value in Rust has an owner.
        There can only be one owner at a time.
        When the owner goes out of scope, the value will be dropped.
    */
}
