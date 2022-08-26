/*
    In some cases, a single value might have multiple owners. For example, in a 
    graph, a node is owned by all the edges that are indicent to it. A node shouldn't be
    cleaned up unless it is disconnected.

    You can enable multiple ownership by using the Rc<T> type which counts the number of
    references to a value that are still in use. If at any point, there are 0 references to a
    value, the value can be cleaned up without any references becoming invalid.

    We use the Rc<T> type when we want to allocate heap memory but do not know which part of 
    our program will finish using the data last at compile time.

    Rc<T> is only for single threaded applications.
*/

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
    /*
        To get this code to compile, we could change Cons to make it hold references instead,
        but then we would need to specify lifetime parameters.
    
        Instead, we can change our definition of List to use Rc<T> in place of Box<T>.
    */
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    /*
        Calling clone increases the reference count by 1. When drop is called (reference goes out of scope),
        the reference count is decreased by 1. b and c contain immutable references to a (if they were mutable,
        Rust's borrowing rules would be violated).

        That is, Rc<T> only allows for immutable references to be created, so only reading, not mutating is possible.
    */
}

