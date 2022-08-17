/*
    Rust closures are anonymous functions you can save in a variable and pass as arguments
    to other functions.

    You can create the closure in one place and then call the closure elsewhere to evaluate
    it in a different context. Closures can capture values from the scope they are defined in.

*/

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        /*
            If the Option<T> is the Some variant, unwrap_or_else returns the 
            value from within the Some. If the Option<T> is the None variant, 
            unwrap_or_else calls the closure and returns the value returned by the closure.
        
            The closure below has no parameters; if it had parameters, they would be 
            between the ||.

            Observe that the closure has access to the scope (self), which is an immutable
            reference to the Inventory instance, unlike a function.

            Closures do not require you to annotate the types of the parameters of the return
            values, unlike functions. Closures are used in narrower context than functions, 
            so types can be inferred by compiler.
        */
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn other_closure_notes() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    /*
        The following are equivalent closure definitions for the same function.
        You can be more verbose if you want.
    */
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    /*
        If the parameters/return type are not made explicit, the first call to the
        closure in a function will allow the compiler to infer the types. Subsequent
        calls to the closure must respect these inferred parameters
    */
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5); // throws an error because compiler inferred type of x to be String
}

fn closure_references_and_moving(){
    /*
        Closures can capture values from their environment in 3 ways:
        1. mutable borrow
        2. immutable borrow
        3. take ownership
    */

    
    immutable_borrow();
    mutable_borrow();
    take_ownership();
   
}

fn mutable_borrow(){
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    
    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    
    /*
        There cannot be any immutable borrow between the closure definition
        and the closure call, since it performs a mutable borrow. The mutable
        borrow is ended after the closure is called.
    */
    println!("After calling closure: {:?}", list);
}

fn immutable_borrow(){
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn take_ownership(){
    /*
        A closure body can:
        1. move a captured value out of the closure
        2. mutate the captured value
        3. neither move nor mutate the value
        4. capture nothing from the environment

        Closures can implement up to 3 traits that define how 

        FnOnce trait applies to all closures that can be called at least once.
        All closures implement at least this trait. A closure that moves out
        captured values from its body will only implement FnOnce, since it
        can only be called once.

        FnMut applies to closures that do not move captured values out of their body,
        but might mutate the captured values.

        Fn applies to closures that do not move captured values out of their body,
        do not mutate captured values, as well as closures that do not capture anything
        about their environment. These can be called multiple time without mutating the
        environment.
    */

    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }

    /*
        Definition of unwrap_or_else accepts the most generic closure (any closure that 
        can be called at least once) and so is flexible.

        Functions can also implement all 3 of the Fn traits, so we can call unwrap_or_else(Vec::new).

        The standard library function sort_by_key defined on slices accepts a closure that implements
        FnMut since it calls the closure multiple times, once for each item in the slice, and doesn't
        capture, mutate, or move out anything from its environment. '
        
        For example, the below closure cannot be called multiple times, since it would be moving value 
        into sort_operations multiple times. Hence, this closure only implements FnOnce and does not 
        implement FnMut as required (causing a compiler error).
    */

    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });

    /*
        We can change the closure to the following, so it implements FnMut
        (this time we do not move a captured value out of the closure).
    */
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
}


