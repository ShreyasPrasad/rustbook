/* ***Packages and Crates***
    Packages: 1 or more crates. Contains a cargo.toml that describes how to build crates.
    Crate: Binary or library. 

        Binary crates compile into executable and must have a main function.

        Library crates do not compile into executable and do not have a main function.
        They define functionality intended to be shared with multiple projects. 
    
    If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a 
    library, both with the same name as the package

*/

/* ***Defining Modules***
    When compiling a crate, the compiler first looks in the crate root file (usually 
    src/lib.rs for a library crate or src/main.rs for a binary crate).

    Modules are declared in the crate root file (main.rs or lib.rs) using the syntax
    mod <mod_name>;
    Compiler searches for module code in src/<mod_name>/mod.rs or src/<mod_name>.rs
    Declaring submodules works the same way (relative to the parent module).

    You can refer to module code using the syntax
    crate::<module_name>::<submodule_name>::<submodule public variable>
*/

/* * ***The pub Keyword***
    To make module public, use syntax
    pub mod;

    To make items in a public module public as well, use pub before their declaration.
*/

/* ***The use keyword***
    Within a scope, the use keyword creates shortcuts to items to reduce repetition of 
    long paths. For example:
    use crate::garden::vegetables::Asparagus 
    means that you only have to write Asparagus now to refer to the submodule variable.
*/

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
