/*
    The Drop trait allows you to customize behavior when a value is about to go out of scope.
    Typically, this can be used to release resources like files or network connections.

    Drop trait is included in the prelude. Explicit calls to drop and not allowed, use
    the prelude function std::mem::drop to drop a value early.
*/

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    c.drop(); // NOT ALLOWED
    drop(c); // ALLOWED
}
