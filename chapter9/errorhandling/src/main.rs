/*
    Rust has a panic! macro that will print a failure message, unwind, and clean up the stack, before quitting.
    panic!("crash and burn");

*/

use std::fs::{self, File};
use std::io::{self, Read};

fn usingresult(){
    /*
        Recall that Result is a two-variant enum:
            enum Result<T, E> {
                Ok(T),
                Err(E),
            }
    
    */
    let f: Result<File, std::io::Error> = File::open("hello.txt");
    let f = match f { // shadowing to assign file handle to f if open call succeeds
        Ok(file) => file,
        Err(error) => match error.kind() { // inner match on the enum returned by the error kind
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    /*
        An alternative to the verbose match primitive is closures (chapter 13).
    */

    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // prints error msg and panics or returns file handle

    /*
        Instead, we can propagate the error to the calling code so it can handle it however it wants to.
    */
    let result: Result<String, io::Error> = read_username_from_file();

    /*
        This pattern is extremely common, so Rust has the question mark operator: ?
    */
    let result: Result<String, io::Error> = read_username_from_file_question_mark();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_question_mark() -> Result<String, io::Error> {
    /*
        If the value of the Result is Ok, it will get returned from the expression,
        otherwise the error will be returned from the whole function.
        This eliminates a lot of boilerplate and is a common Rust pattern.
        Note that the ? operator can only be used in functions with a compatible return type.
        ? can be used with both Result and Option enums (None is returned upon encountering error)
    */
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_elegant() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt") // directly return Result using fs function
}

fn main() {
    usingresult();
}
