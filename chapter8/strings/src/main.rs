/*
    Rust has only one string type in core language, which is the string slice str, commonly borrowed as &str.
    String literals are stored in the program binary and are therefore slices.

    String type is a growable, mutable UTF-8 encoded string.
*/

fn strings(){
    let mut s = String::new(); // empty string

    let s = String::from("initial contents"); // prepopulated string

    let mut s = String::from("foo"); // mutable string allows us to push string slices
    let s1 = "bar";
    s.push_str(s1); // push_str takes a string slice param since we don't want to take ownership of the parameter
    s.push('l'); // push a single character

    /*
        To combine existing strings, use + operator or the format! Macro.
    */

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    /*
        s1 has been moved above because add method has the signature:
            fn add(self, s: &str) -> String
        
        1. add takes ownership of self when creating a new string, hence s1 becomes invalid
        
        2.  Notice that the type of s in the signature is &str but the argument passed
            when creating s3 is of type &String. 
            This compiles because Rust performs deref coercion, turning &s2 into &s2[..]
    */

    let t = format!("{}-{}", s2, s3); // more convenient way to add strings

    let hello = String::from("Здравствуйте");
    let hello_first_char = &hello[0..4];
    /*
        This string consists of 24 bytes (because of unicode scalar values) but has a "length" of 12.
        Indexing strings is now allowed for this reason (the index may not be what you expect length-wise).
        Rust allows you to slice a part of string that represents a valid unicode char, otherwise panic is thrown.
        So in summary, single index not allowed, a slice is conditionally allowed.
    */

    // method to iterate over the 6 chars of a string slice; refer to grapheme clusters to get only the 4 hindi symbols
    for c in "नमस्ते".chars() { 
        println!("{}", c);
    }
}
fn main() {
    println!("Hello, world!");
}
