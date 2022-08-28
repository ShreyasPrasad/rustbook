/*
    To share data between threads, Rust also has a safe lock implementation.
    Mutex<T> is a type a smart pointer; more specifically, calling 
    lock returns a smart pointer called MutexGuard, wrapped in a LockResult that we
    handle using unwrap. The MutexGuard smart pointer implements Deref to point to
    inner data. The smart pointer also has a Drop implementation that releases the
    lock automatically when the MutexGuard goes out of scope.
*/

use std::sync::Mutex;

fn introducing_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m); // prints the updated value of 6
}

fn sharing_mutex_between_threads(){
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        /*
            We give all the threads the same closure: one that moves
            the counter into the thread.

            This doesn't compilebecause the counter value is moved in the 
            first iteration of the loop; this means we can't move the
            ownership of counter into multiple threads.
        */
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    /*
        Call join on each handle to make sure all 10 threads finish.
    */
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn multiple_ownership_with_multiple_threads(){
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        /*
            To solve the compile error from before,
            wrap the Mutex in a Rc to allow multiple
            threads to own refernces to the counter.

            However, this still doesn't compile, because
            Rc is not safe to share across multiple threads.
            This is since Rc doesn't use concurrency primitives
            to atomically count the number of active references
            that still exist to the given value.
        */
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn using_arc(){
    /*
        We can use Arc<T> to atomically make reference counts.
        The reason Rc and Arc exist separately is because thread safety
        comes with a performance penalty that you only pay when you need to.
        
        Mutex<T> provides interior mutability like Cell. We use it to mutate
        the contents of an Arc<T>
    */
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
