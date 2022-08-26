/*
    Interior mutability is a design apttern that allows you to mutate data even when there
    are mutable references to the data. To mutate data, the pattern uses unsafe code inside
    a data structure to bend Rust's usual rules.

    This pattern is applied using RefCell<T> which will panic if the borrowing rules are 
    violated at runtime (no compile time checks). 

    The advantage of this pattern is that is allows certain memory-safe scenarios, that the
    compiler would otherwise object to.

    RefCell<T> is only for use in single threaded scenarios. RefCell<T> has a single owner,
    just like Box<T>.

    Note that Rust does not allow mutable borrows of immutable variables, like the following.

    let x = 5;
    let y = &mut x;

    RefCell<T> allows mutable borrows checked at runtime, so you can mutate the value inside
    RefCell<T> even when RefCell<T> is immutable. Again, RefCell cannot be used for multithreading,
    use Mutex<T> instead.
*/

pub trait Messenger {
    /*
        Accepts an immutable reference to self.
    */
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        /*
            Without RefCell, this send method would throw an error because it attempts
            to perform a mutating operation on an immutable reference to self.

            borrow method returns Ref<T> and borrow_mut method returns RefMut<T>, both of
            which implement Deref.

            Whenever borrow_mut is called, it increases a count of the number of mutable 
            references to the data. 

            Whenever borrow is called, it increases a count of the number of immutable 
            references to the data. 

            When a reference goes out of scope, these counts go down by 1.

            RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time,
            erroring at runtime if this is violated at any time.
        */
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

/*
    You can combine RefCell<T> with Rc<T> to have multiple owners of some data, 
    and mutate the data.
*/

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    /*
        Both a and value have ownership of the RefCell.
    */
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    /*
        a is of type Rc<T>, and thus b and c both refer to a.
    */

    *value.borrow_mut() += 10;

    /*
        Automatic dereferencing ensures that dereferencing value returns
        the contained RefCell<i32>.
    */

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
