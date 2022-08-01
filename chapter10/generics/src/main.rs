/*
    Rust has generics, which allows the duplication of concepts to be handled.
*/

/*
    We can use generics in function definitions to replace varius parts of the
    function definition, including parameter types and the return type.

    Below, is a generic largest function. This won't compile because of the 
    ambiguous use of > (greater than operator). We must make sure that the 
    the type T supports ordering. 

    No runtime cost is paid for using generics; monomorphization is used during
    compile time to turn generic code into type-specific equivalents by filling
    in concrete types.
*/

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
    We can also use generics in nenum (like Option and Result), struct
    and method implementations.
    
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
*/

struct Point<T, U> {
    x: T,
    y: U, // This allows y to have a different type than x.
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p1 and p2 are invalid here
}
