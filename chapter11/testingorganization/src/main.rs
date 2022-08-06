/*
    Use cargo test --help to see available test options.

    Tests can be run in parallel, using multiple threads, or consecutively.
    For parallel running tests, you must make sure that tests do not depend
    on each other or some shared state.

    To not use any parallelism:

        cargo test -- --test-threads=1

    To show standard output:

        cargo test -- --show-output

    To run a specific test:

        cargo test <test function name>

    To run multiple tests using a part of a test name:

        cargo test add

    To ignore a certain test:

        #[ignore]

    To run ignored tests:

        cargo test -- --ignored

    To run all tests, including ignored tests:

        cargo test -- --include-ignored
*/


/*
    Unit tests:

    Test each unit of code in isolation, placed in same file of the code
    they are testing. The convention is to create a module named test to 
    contain with test functions and annotate the module with #[cfg(test)]

    The cfg attribute ensures that the following code is only compiled if 
    given a certain configuration option, like test.

    Rust's privacy rules do allow testing of internal functions, but it's up
    to you if they should be tested.
*/

/*
    Integration tests:

    External to any single library, using your library in the same way any other
    code/client would. They call functions from your library's public API.

    tests directory and top level of project directory (next to src) os required.
    Cargo looks for integration tests in this directory.


*/
