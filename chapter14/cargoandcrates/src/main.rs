/*
    ***Profiles***

    Cargo has 2 main release profiles: dev and release. These profules allow for control over 
    compiler options.

    Dev is used when simply running cargo build. Release is used when running cargo build --release.
    
    You can override defaults for profile settings in Cargo.toml.

    [profile.dev]
    opt-level = 0

    [profile.release]
    opt-level = 3

    opt-level specifies the level of optimizations made; more equal faster code, but slower compile time.
*/


/*
    ***Crates***

    Documentation comments made with 3 slashes: ///
    Crate authors typically include panics, errors, and safety of the code. These comments can include 
    code examples that will run when cargo test is invoked.

    Module/crate comments are made with //!. These show up on the front page of the crate, as generated 
    by rustdoc.
*/

/*
    ***Workspaces***

    Cargo workspaces allow you to manage multiple related packages. A workspace is a set of packages that 
    share the same Cargo.lock and output directory. Use workspaces as your project grows.

    In the root directory, we create a Cargo.toml file with the following:

    [workspace]

    members = [
        "<package_1_name>",
        "<package_2_name>",
        ...
    ]

    Then, we run cargo new <package_1_name> to create the new child package. Running cargo build from anywhere
    creates the output in /target directory relative to the root.

    Running cargo test runs tests for all packages specified as workspace members. Note that there is only one 
    Cargo.lock file at the top level, so all crates share the same dependencies and are always compatible with
    each other.

    You can install cargo packages using cargo install <package_name>.
*/

