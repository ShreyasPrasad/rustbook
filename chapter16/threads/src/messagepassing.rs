/*
    Messages passing is a popular safe concurrency model.
    “Do not communicate by sharing memory; instead, share memory by communicating.”
    Rust provides channels to send data from one thread to another.
    
    A channel has a transmitter and a receiver. A channel is closed if either the
    transmitter or receiver half is dropped.
*/

use std::sync::mpsc;

fn introducing_channels() {
    /*
        The following creates a new channel, mpsc stands for multiple producer, single
        consumer. That means, we can multiple senders but only one receiver in Rust.
        
        Tuple of transmitter, receiver pair returned.
    */
    let (tx, rx) = mpsc::channel();

    /*
        Use move to move tx into the closure so the spawned thread owns tx. The spawned
        thread needs to own the transmitter to be able to send messages through the channel.

        send method returns a Result<T,E> type, so if the receiver had already been drop, the
        send operation will return an error for the sender transmitting thread to handle.
    */
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    /*
        Blocks until value is received. Use try_recv if you don't want to block. try_recv is useful
        if the thread needs to do additional work (not block) while waiting for messages. You can also
        place try_recv in a loop to control when you check for messages.
    */
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn channels_and_ownership(){
    let (tx, rx) = mpsc::channel();

    /*
        The following throws a compile-time error because we are trying to use a value in a thread after
        it could potentially be altered by the other thread. This is because the send function takes
        ownership of its parameter, and when it is moved the receiver takes ownership of it.
    */
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn channels_and_sending_multiple_values(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    /*
        For each value received, we’re printing it. When the channel is closed, iteration will end.
        This for loop continues to wait for received values.
    */
    for received in rx {
        println!("Got: {}", received);
    }
}

fn channels_and_multiple_producers(){
    let (tx, rx) = mpsc::channel();
    /*
        We can use tx.clone to obtain a new transmitter.
    */
    let tx1 = tx.clone();

    /*
        This thread takes ownership of and uses tx1 transmitter.
    */
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    /*
        This thread takes ownership of and uses tx transmitter.
    */
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    /*
        The receiver waits for messages from all transmitters.
    */
    for received in rx {
        println!("Got: {}", received);
    }
}
