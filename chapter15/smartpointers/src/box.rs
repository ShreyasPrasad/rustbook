/*
    Recall that references are indicated by & and borrow the value they point to.
    Rust also has pointers that allow for multiple owners for data, and clean up data
    when no owners remain. Smart pointers own the data they point to.

    The most common smart pointers are:

    Box<T> for heap values
    Rc<T> a reference counting type that enables multiple ownership
    Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces borrowing rules
    at runtime instead of compile time.
*/


fn box(){
    /*
        Use box when:
        1. Size of type not known at compile time.
        2. When you have a large amount of data and do not want to copy it when transferring ownership.
        3. When you want to own a value that implements a particular trait.
    */
    let b = Box::new(5);
    println!("b = {}", b);
    /*
        Just like any owned value, when b goes out scope, it will be deallocated. This occurs for both the
        Box (stored on the stack), and the data it points to (stored on the heap).
    */
    /*
        The cons list is a common functional programming data structure: (1, (2, Nil))
        Rust doesn't allow this recursive enum definition because it cannot calculate its size at compile time.

        The compiler needs some indirection (i.e a pointer like Box) to hold the recursive value, because the pointers
        value is of fixed size, unlike a List.
    */
    // Not allowed!!!
    enum List {
        Cons(i32, List),
        Nil,
    }
    // Allowed!!!
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
}



