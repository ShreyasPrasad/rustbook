/*
    Each file in the tests directory is a separate crate.
    Cargo treats the tests directory specifically and compiles files in this
    directory only when we run cargo test.

    To run all tests in an integration test file, run cargo test --test integration_test
*/

use adder; // import library crate adder

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
