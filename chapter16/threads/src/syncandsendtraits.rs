/*
    Rust has few concurrency constructs. There are 2 important ones though.
    The std::marker traits Sync and Send.

    The Send marker trait indicates that ownership of values of the type 
    implementing it can be transferred between threads. Almost every Rust type
    is Send, but there are some exceptions, like Rc<T> (which is meant for
    single-threaded situations). Any type composed ot Send types is automatically
    marked as Send as well, aside from raw pointers.

    The Sync marker traits indicates that its safe for the type implementing it to
    be referenced from multiple threads. Any type T is Sync if &T (immutable ref to T)
    is Send, meaning the reference can be sent safely to another thread. Primitive
    types are Sync, and types composed entirely of types that are Sync are also Sync.
*/
