/*
    Rust allows memory leaks by using Rc<T> and RefCell<T> since its possible to create
    references where items refer to each other in a cycle. This creates memory leaks
    because the reference count of each item in the cycle will never reach 0, and thus
    the values will never be dropped.
*/

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    /*
        We want to modify the List value a Cons variant is pointing to, so we wrap
        it in an RefCell
    */
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    /*
        tail method to make it convenient for us to access the second item if we have a Cons variant
    */
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    /*
        We modify a so it points to b instead of Nil, creating a cycle.
        a -> b -> a... and so forth. You must guard against reference 
        cycles yourself.
    */
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

/*
    Rc<T> instance is only cleaned up if its strong_count is 0.

    You can also create a weak reference to the value within an Rc<T> instance by 
    calling Rc::downgrade and passing a reference to the Rc<T>

    Strong references are how you can share ownership of an Rc<T> instance. 
    Weak references don’t express an ownership relationship, and their count 
    doesn't affect when an Rc<T> instance is cleaned up. 

    Calling Rc::downgrade increases the weak_count by 1, not the strong_count.
    The weak_count does not need to be 0 for the instance to be cleaned up.

    Since it is possible that after calling Rc::downgrade, the value doesn't exist
    anymore, you must call Rc::upgrade which will return an Option<Rc<T>>, populated
    with Some<Rc<T>> if the value still exists.
*/

use std::cell::RefCell;
use std::rc::Rc;

/*
    We create a graph structure in which a node owns references to its children.
*/
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // set the leaf's parent to branch
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    /*
        We clone the Rc<Node> in leaf and store that in branch, meaning the Node in 
        leaf now has two owners: leaf and branch.

        Note that it's easy to go from branch to leaf, but the leaf knows nothing about
        its parent (branch). Thus, we add a parent field to the Node struct definition.
        To avoid a reference cycle, we make this a weak reference. This makes sense
        logically; a child should not own its parent, but should be aware of it.

        Using Weak<T> avoids a reference cycle.
    */
}
