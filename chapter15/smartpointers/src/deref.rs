/*
    Implementing the Deref trait allows you to customize the behaviour
    of the dereference (*) operator.
*/

use std::ops::Deref;

fn pointers(){
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // y is a pointer that can be dereferenced, comparing y to 5 is disallowed
}

fn box_as_a_reference(){
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // we can use * operator because Box implements Deref trait
}

struct MyBox<T>(T); // create custom box type with a new function

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    /*
        The deref method gives the compiler the ability to take a value of any type that implements 
        Deref and call the deref method to get a & reference.

        Behind the scenes, Rust actually runs *(y.deref())

        If the deref method returned the value directly instead of a reference to the value, the 
        value would be moved out of self, which is not what we want.
    */
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
    Deref coercion converts a type that implements the Deref trait into a reference of another type.
    For example, deref coercion converts &String to &str because String implements the Deref trait 
    and returns &str.

    Rust does deref coercion in the following 3 cases:

    1. From &T to &U when T: Deref<Target=U>
    2. From &mut T to &mut U when T: DerefMut<Target=U>
    3. From &mut T to &U when T: Deref<Target=U> (the reverse is not possible because of borrowing rules)
*/

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn call_hello(){
    let m = MyBox::new(String::from("Rust"));
    /*
        deref is called twice:
        once to convert &MyBox<String> to &String
        once to convert &String to &str
        which then matches the function signature of hello
    */
    hello(&m);
}

