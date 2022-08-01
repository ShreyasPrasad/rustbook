/*
    Traits define shared behaviour, like interfaces in other languages.
    Trait definitions are a way to group method signatures.
    Different types share the same behaviour if we can call the same methods
    on all of those types.
*/

/*
    We declare a trait as follows, which has a shared method for the types that
    implement this trait.
*/

pub trait Summary {
    /*
        Default implementation, called if type doesn't implement its own.
        Default implementation can call trait methods, which may not have
        default implementations.
    */
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/*
    for keyword specifies type we want to implement trait for.
    We must define the method signatures that the trait has defined in the impl block.
    Note that the trait and the type must be brought into scope for the trait methods
    on the type to be used. Also, we cannot define external traits on external types.
    This is called the orphan rule.
*/
impl Summary for NewsArticle { 
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*
    This method accepts any type that implements the trait Summary
*/

pub fn notify(item: &impl Summary) {}

/*
    This method is the longer syntax that is equivalent to the above method.
    We declare the generic type that implements a specific trait, and use
    it as a parameter to the notify function.
*/

pub fn notify<T: Summary>(item: &T) {}

/*
    We can also specify multiple trait bounds with the + syntax.
*/

pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}

/*
    This syntax makes function signatures difficult read, so 
    we can use where clauses in the signature to separate parameter
    declaration and their bound traits.
*/

pub fn notify<T>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

/*
    We can also have function with trait return types.
    However, they can only return a single type that implements
    the given trait, not multiple types.
*/

pub fn notify(s: &String) -> impl Summary {}

/*
    To make sure that only Copyable types that implement a certain trait
    are accepted, use the Copy trait in the function signature to enforce it.
    Below, largest requires ownserhip of list, so we require the Copy trait.  
*/

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {}


/* 
    Lastly, we can use traits to conditionally implement a trait; that is, 
    implement a method on a generic type only if it implements certain traits.
*/

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
