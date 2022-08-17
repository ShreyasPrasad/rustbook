/*
    Iterators allow you to perform some task on a sequence of items.
    An iterator is responsible for the logic of iterating over each
    item and determining when the sequence has finished.

    Rust iterators are lazy, meaning that have no effect until you 
    call methods that consume the iterator to use it up.
*/

fn using_iterators(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter(); // creates an iterator, and stores it in a variable
    /*
        Iterators allow us to use the for in syntax with any data structure, not just 
        a vector. This means, we do not need to index through the structure.
    */

    for val in v1_iter { // uses the iterator
        println!("Got: {}", val);
    }
}

/* 
    All iterators implement the Iterator trait.

pub trait Iterator {
    type Item; // associated type, see chapter 19

    /*
        The Iterator trait only requires implementors to define one method: the 
        next method, which returns one item of the iterator at a time wrapped in 
        Some and, when iteration is over, returns None.

        We can call the next method on iterators directly. See lib.rs.
    */

    fn next(&mut self) -> Option<Self::Item>;

    /*
        There are a number of methods with default implementations. 
        
        Methods that call next are called consuming adaptors, because calling them 
        uses up the iterator. One example is sum, which takes ownership of the iterator,
        consumes it by repeatedly calling next, and adds the items to a running total.
        We cannot use v1_iter after sum, because it takes ownership.

        let total: i32 = v1_iter.sum();

        Methods that produce iterators by changing some aspect of the original iterator are 
        called iterator adaptors. One example is map which takes a closure that is called 
        on each element of the original iterator to create the corresponding element in the
        new iterator.

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        Another popular iterator adaptor is filter. filter(<closure>) returns the original elements 
        when the closure predicate evaluates to true. Thus, it can't give away ownership of the 
        elements to the closure, so it must pass the elements by reference.

        let v2: Vec<&u32> = v1.iter().filter(|&x| *x < 2).collect(); // does not take ownership of v1
        let v2: Vec<u32> = v1.into_iter().filter(|&x| x < 2).collect(); // takes ownership of v1

        Since iterators are lazy, a result vector is only created when the collect method is called.
    */
}
*/

fn main() {
    using_iterators();
    /*
        Use iterators where possible instead of loops, they are slighty more efficient (zero cost 
        abstraction).
    */
}
