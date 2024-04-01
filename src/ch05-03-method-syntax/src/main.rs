#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block for extending Rectangle struct methods
impl Rectangle {
    // &self is a short for self: &Self
    // Within the impl block Self is an alias for the type that the impl block is for
    // Note that we still need to use the & in front of the self shorthand to indicate
    // that this method borrows the Self instance, just as we did in rectangle: &Rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Note that we can choose to give a method the same name as one of the struct’s fields.
    fn width(&self) -> bool {
        self.width > 0
    }

    // All functions defined within an impl block are called associated functions
    // because they’re associated with the type named after the impl. We can define
    // associated functions that don’t have self as their first parameter (and thus are not methods)
    // because they don’t need an instance of the type to work with.
    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    // These are often called new, but new isn’t a special name and isn’t built into the language.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Multiple impl blocks are valid
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Calling an associated function that is not a method
    let sq = Rectangle::square(3);
    println!("The area of the square is {}.", sq.area());
}
