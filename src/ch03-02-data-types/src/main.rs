use std::io;

fn main() {

    // Integers
    // -------
    // Length | Signed | Unsigned
    // 8-bit | i8 | u8
    // 16-bit | i16 | u16
    // 32-bit | i32 | u32
    // 64-bit | i64 | u64
    // 128-bit | i128 | u128
    // arch | isize | usize
    let a = 2147483647; // i32
    let b: u64 = 18446744073709551615; // u64
    println!("a: {a}, b: {b}");

    // Floating-point
    // -------------
    let x = 2.0; // f64 - double precision
    let y: f32 = 3.0; // f32 - single precision
    println!("x: {x}, y: {y}");

    // Numeric operations
    // -----------------
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;
    println!("sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, truncated: {truncated}, remainder: {remainder}");

    // Boolean (1 byte in size = 8 bits)
    // --------------------------------
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t: {t}, f: {f}");

    // Character
    // --------
    // Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes.
    // Rust’s char type is *four bytes in size* and represents a Unicode Scalar Value, which means it can represent a
    // lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji;
    // and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to
    // U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode,
    // so your human intuition for what a “character” is may not match up with what a char is in Rust.
    // More details in chapter 8: https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");

    // Compound Types
    // -------------
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // Tuples
    // -----
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:?}", tup);

    // The variable tup binds to the entire tuple because a tuple is considered a single compound element.
    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
    let (x1, y1, z1) = tup;
    println!("The value of x1 is: {x1}");
    println!("The value of y1 is: {y1}");
    println!("The value of z1 is: {z1}");

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred} {six_point_four} {one}");

    // The tuple without any values has a special name, unit.
    // This value and its corresponding type are both written () and represent an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value.

    // Arrays
    // -----
    // Another way to have a collection of multiple values is with an array.
    // Unlike a tuple, every element of an array must have the same type.
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.
    // We write the values in an array as a comma-separated list inside square brackets:
    let arr = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);

    // Arrays are useful when you want your data allocated on the stack rather than the heap
    // or when you want to ensure you always have a fixed number of elements.
    // An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided
    // by the standard library that is allowed to grow or shrink in size. If you’re unsure whether
    // to use an array or a vector, chances are you should use a vector.

    // Useful when you know before hand the size of the array and you know it won't change
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months: {:?}", months);

    // If we need to define the type of the elements and size:
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr2: {:?}", arr2);

    // Initializing an array containing a certain value N times
    let arr3 = [3; 5];
    println!("arr3: {:?}", arr3);

    // Accessing array elements
    let first = arr[0];
    let second = arr[1];
    println!("{first} {second}");

    // Invalid array index
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // This may panick if index is out of bounds
    // This is an example of Rust’s memory safety principles in action.
    // In many low-level languages, this kind of check is not done, and when you provide an incorrect index,
    // invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting
    // instead of allowing the memory access and continuing
    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
