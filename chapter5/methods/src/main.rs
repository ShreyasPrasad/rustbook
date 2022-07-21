#[derive(Debug)] // trait that we apply the derive attribute to, allowing values to be printed when debugging
struct Rectangle {
    width: u32,
    height: u32,

}

//All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl
impl Rectangle { // Everything within this impl block will be associated with the Rectangle type
    fn area(&self) -> u32 { // &self is actually short for self: &Self.
        /*
            Methods must have a parameter named self of type Self for their first parameter. 
            Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self 
            mutably, just as they can any other parameter.

             Having a method that takes ownership of the instance by using just self as the first parameter 
             is rare; this technique is usually used when the method transforms self into something else 
             and you want to prevent the caller from using the original instance after the transformation.
        */
        self.width * self.height
    }

    fn width(&self) -> bool { // method with the same name as a field (distinguished using parenthesis)
        self.width > 0
    }
  
    fn can_hold(&self, other: &Rectangle) -> bool { // second parameter is a reference to another rectangle
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle { // separate impl block for same type
    fn square(size: u32) -> Rectangle { // associated function without self as first parameter
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1); // we don't want to give the macro ownership of rect1

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    /*
        Rust has a feature called automatic referencing and dereferencing. 
        Calling methods is one of the few places in Rust that has this behavior.
        When you call a method with object.something(), Rust automatically adds in &, &mut, or * 
        so object matches the signature of the method.
        
        p1.something()
        (&p1).something()

        are the same in Rust
    */

    let sq = Rectangle::square(3); // calling an associated function, just like String::from
}
