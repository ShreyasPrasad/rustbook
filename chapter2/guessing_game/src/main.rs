/*
The prelude is the list of things that Rust automatically imports into every Rust program. 
It’s kept as small as possible, and is focused on things, particularly traits, which are 
used in almost every single Rust program.

*/

use std::io; // importing a standard library not in the prelude
use rand::Rng; // The Rng trait defines methods that random number generators implement
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // inclusive of bounds
    /*
        gen function contained in Rng trait, which is imported above
    */

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // make a variable mutable
        /*
            ::new indicates that new is an associated function of the String type.
            An associated function is a function that’s implemented on a type, in this case String
        */

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*
            The stdin function returns an instance of std::io::Stdin, which is a 
            type that represents a handle to the standard input for your terminal.
            The & indicates that this argument is a reference.
            You need to write &mut guess rather than &guess to make it mutable.

            read_line also returns a Result value, an enumeration, often called an enum, 
            which is a type that can be in one of multiple possible states. 
            We call each possible state a variant. Result's variants are Ok and Err

            If this instance of Result is an Err value, expect will cause the program to 
            crash and display the message that you passed as an argument to expect
        */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        /*
            Variable shadowing allows us to reuse the previous variable name with a different
            type.
            u32 represents unsigned 32 bit integer, i32 is signed.
            Note that since parse returns a Result type which is an enum with 2 variants, 
            we can apply a match expression to continue the loop when any input error arises;
            underscore is a catchall value.
        */

        println!("You guessed: {guess}");

        /*
            The project we’ve been building is a binary crate, which is an executable. 
            The rand crate is a library crate, which contains code intended to be used in 
            other programs and can't be executed on its own.

            When you build your project in the future, Cargo will see that the Cargo.lock 
            file exists and use the versions specified there rather than doing all the work 
            of figuring out versions again. This lets you have a reproducible build automatically.
        */

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        /*
            Ordering is an imported enum type with 3 variants.
            A match expression is made up of arms. An arm consists of a pattern to match against, 
            and the code that should be run if the value given to match fits that arm’s pattern.
        */
    }
}
