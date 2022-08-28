/*
    Threads complicate memory management because they can lead to race conditions,
    deadlocks, and other mutual contention bugs.

    The Rust standard library uses a 1:1 model of thread implementation, whereby a 
    program uses one operating system thread per one language thread.
*/

use std::thread;
use std::time::Duration;

fn introducing_threads() {
    /*
        thread::spawn takes a closure which is the thread we wish to run.
    */
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    /*
        The following code terminates earlier than the spawned thread code usually;
        this means the spawned thread is prematurely stopped most of the time.

        We can fix this problem by using a JoinHandle, which waits for a thread to finish.

        Note that calling join on the handle blocks the thread currently running until
        the thread represented by the handle terminates.
    */
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn moving_to_threads() {
    /*
        We'll often use the move keyword with closures passed to thread::spawn because the 
        closure will then take ownership of the values it uses from the environment.

        In the following example, we get the compile time error "may outlive borrowed value `v`"
        because  Rust can’t tell how long the spawned thread will run, so it doesn’t know if 
        the reference to v will always be valid.

        For example, drop(v) may execute in the main thread before the spawned thread executes
        its print, leading to the use of an invalid reference in the spawned thread.

        To fix this, prefix the closure with the move keyword.
    */
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // oh no!

    handle.join().unwrap();
}
