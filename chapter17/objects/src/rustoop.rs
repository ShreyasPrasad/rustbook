/*
    Rust emulates certain object oriented patterns.

    Objects contain data and behaviour:
    structs contain the data and the impl blocks provide methods on structs and enums.    
*/

/*
    Encapsulation to hide implementation details:
    The use of the pub keyword can hide certain module types, functions, and methods,
    and allows us to define a contract for a public interface.

    The following struct is public, so it can be used by other modules, but its fields
    are private, allowing manipulation only through the public methods below.
*/
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

/*
    Inheritance as a type system:
    Inheritance allows objects to inherit elements from another (parent) object's definition.
    Inheritance is chosen for code reuse and to enforce polymorphism between types.

    In Rust, generics abstract over different possible types and trait bounds to impose 
    constraints on what those types must provide.

    Consider an example of a GUI library in which we have a class named Component which has a
    method name draw on it. Other classes, such as Button and Image would inherit from Component,
    and thus override the draw method to define their custom behavior.

    To do this, we can define a trait named Draw that will have one method named draw. Then, we
    can define a vector that takes a trait object. A trait object points to both an instance of a 
    type implementing our specified trait, and a table used to look up trait methods on that type
    at runtime. We can specify a trait object by using some type of pointer (like Box) followed by
    the dyn keyword, and then including the relevant trait.
*/

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/*
    This is better than defining a struct that uses a generic type parameter with trait bounds such
    a generic type parameter can be substituted with only one concrete type at runtime, whereas
    trait objects allow for multiple concrete types to fill in for the trait object at runtime.
*/

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

/*
    This pattern allows us to create types in the future that implement the Draw trait without changing
    our original code. It also alerts us, with a nice compiler error, when we try to add a component
    to our Screen that does not implement the Draw trait.

    With generic types, the compiler generates specific implementations of functions and methods for
    each concrete type that are used in place of a generic type parameter before runtime. This is known
    as monomorphization and the resulting code does static dispatch, where the compiler knows the method
    that ie being called at compile time. Thus, no runtime cost is incurred.

    On the other hand, dynamic dispatch occurs when the compiler can't tell at compile time which method
    you're calling. The compiler doesn't know all the types that might be used with the code that's using
    trait objects, so it doesn't know which type to call. Instead, ar runtime, Rust uses the pointers intisde
    the trait object to know which method to call, but this results in a runtime lookup cost.
*/
