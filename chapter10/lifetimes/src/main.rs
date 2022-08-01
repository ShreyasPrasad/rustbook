/*
    Lifetimes are a type of generic; they specify the scope for which a reference is valid.
    
    Most of the time, lifetimes are implicit and inferred. Lifetime annotations are only
    required when multiple types are possible.
*/

/*
    The main aim of lifetimes is to prevent dangling references
*/

fn danglingReferences(){
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    /*
        Borrowed value does not live long enough.
        x will go out of scope when at the end of the inner scope.
        r is still valid in the ohter scope.
        Rust determines this code is invalid using a borrow checker:
        At compile time, Rust sees that r has a lifetime of 'a but it refers to memory with a 
        lifetime of 'b.
    */
}

/*
    The following will not compile, because Rust does not know the lifetime of the result
    relative to the lifetime of the parameters x and y. To fix, this error, we need to
    define the relationship between the references so the borrow checker can perform its
    analysis.
*/

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
    Lifetime annotations do not change how long the references live; they describe the 
    relationships of the lifetimes of multiple references without affecting these lifetimes.
*/

fn lifetimeannotations(){
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
    /*
        If we have 2 parameters to a function both annotated with 'a,
        this indicates that both live as long as that generic lifetime.
    */
}

/*
    Similar to generic type parameters, we need angled brackets to declare generic lifetime
    parameters. This function signature expresses that the returned reference will be valid
    for as long as both x and y are valid. This means that the lifetime of the result is valid
    for the smaller of the lifetimes of x and y.
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn valid_lifetime(){
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result); 
    }
    /*
        result is valid until the end of the inner scope, this compiles.
    */
}

fn invalid_lifetime(){
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); 
    /*
        result is valid until the end of the inner scope, so this
        does not compile.
    */
}

/*
    Depending on what our function does, we can change it's signature.
    Note that we do annotate the parameter y with a lifetime because it 
    does not relate to the lifetime of x or the return value.
*/

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

/*
    We cannot use a generic lifetime type solely in the return type like
    the following example shows. result goes out of scope and we are also
    trying to return a reference to result from the function. Does not compile.
*/

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

/*
    Structs can also have lifetime annotations in definitions.
*/

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn usingstructlifetime(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

/*
    The compiler also has a set of built-in rules that allow for explicit lifetime
    annotations to be elided. These are based on common use patterns and are subject
    to change. 
    
    The 3 rules are:
     
    1. A unique lifetime parameter is assigned to each parameter that's a reference.
       So a function with 2 reference parameters gets 2 lifetime parameters etc.

       fn foo<'a>(x: &'a i32)
       
    2. If there is exactly one input lifetime parameter, that lifetime is assigned to 
       all output lifetime parameters (there may be multiple).

       fn foo<'a>(x: &'a i32) -> &'a i32

    3. If there are multiple input lifetime parameters, but one of them is &self or &mut self
       (because this is a method), the lifetime of self is assigned to all output lifetime 
       parameters.

    If after working through all three rules, the compiler has npt figured out the lifetimes
    of all references in the function signature, a compiler error is raised.
*/

/*
    Lifetime names for struct fields always need to be declared after the impl keyword and then used 
    after the struct’s name, because those lifetimes are part of the struct’s type.

    But because of elision rules, we usually don't need to explicitly refer to the lifetime parameter often.
*/

impl<'a> ImportantExcerpt<'a> {
    // first elision rule
    fn level(&self) -> i32 {
        3
    }

    // first and third elision rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

/*
    The static lifetime is a special lifetime that lives for the entire duration of the program
    (stored in program binary).
    
    The lifetime of all string literals is 'static.
*/

fn staticlifetime(){
    let s: &'static str = "I have a static lifetime.";
}
