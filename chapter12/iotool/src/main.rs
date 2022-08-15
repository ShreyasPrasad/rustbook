/*
    Given Rust's speed and safety, it is natural to write command line tools in it.
    We will write a simplified version of grep.
*/

/*
    Bring parent module into scope.
*/
use std::env;
use std::process;

use iotool::Config;

fn main() {
    /*
        Iterators produce a series of values and we can call the collect method on an
        iterator to turn it into a collection, such as a vector.

        Will panic, if any argument is invalid Unicode.
        First argument is program name, relative to root program directory.

        Pass arguments as follows: cargo run -- <first arg> <second arg> ...
    */
    let args: Vec<String> = env::args().collect();
    /*
        The unwrap_or_else takes a closure (anonymous function) as an argument, and 
        invokes it with the inner value of the error it receives from the Result
    */
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    /*
        run doesn't return a useful Ok value, so we only care about catching errors using
        let syntax.
    */
    if let Err(e) = iotool::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
