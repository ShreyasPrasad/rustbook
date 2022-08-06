/*
    Test modules can have test functions (annotated with #[test]) and non-test functions.
    Tests fail when something in the test function panics.

    assert! macro panics if the condition passed to it fails.
    assert_eq! tests equality using == between its 2 arguments.
    assert_ne! uses the != operator.

    #[should_panic] tests that a panic is raised by the test body. 

    You can return a Result<T, E> from a test; any error returned will signal test failure.
    This is useful when we wish to use the question mark operator for error catching.
 */

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn will_fail() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn assert_eq_success() {
        assert_eq!(4, 4);
    }

    #[test]
    fn assert_failure() {
        assert!(4 == 3, "The value 4 was not equal to the value '{}'", 3);
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        assert!(100>101);
    }
}
