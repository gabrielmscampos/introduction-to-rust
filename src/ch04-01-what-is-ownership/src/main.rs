fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // There is a natural point at which we can return the memory our String needs to the allocator:
    // when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us.
    // This function is called drop, and it’s where the author of String can put the code to return the memory.
    // Rust calls drop automatically at the closing curly bracket.
    {
        let _ = String::from("hello"); // _ is valid from this point forward
        // do stuff with _
    } // this scope is now over, and _ is no longer valid

    // Variables and Data Interacting with Move
    // .......................................
    // bind the value 5 to x; then make a copy of the value in x and bind it to y
    // This is indeed what is happening, because integers are simple values with a known,
    // fixed size, and these two 5 values are pushed onto the stack.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // When we assign s1 to s2 all data from s1 is copied to s2
    // i.e. a string is made up of [pointer, length, capacity]
    // so s2 and s1 points to the same memory address in the heap
    // the data the pointer is referencing is not duplicated
    let s1 = String::from("hello");
    let s2 = s1;

    // Going out of the scope would be a problem because
    // both s1 and s2 would try to free the same memory (double free error)
    // what Rust do to ensure memory safety is considering s1 no longer valid
    // after `let s2 = s1`, i.e. Rust doesn't need to free anything when s1 goes out of scope
    // This is similar to shallow copy vs deep copy, but since rust invalidates the first variable
    // instead of begin called shallow copy, it's know as a *move*

    // Adding the following line will give an error, because s1 is not longer
    // valid and to use that variable we could create s2 cloning s1 instead of assigning
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // Variables and Data Interacting with Clone
    // ........................................
    // For creating a deep copy (i.e. copying the data in the heap and creating a new pointer) we can use clone:
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Ownership and Functions
    // ....................
    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
    main_ownership_funcion();

    // Return Values and Scope
    // ......................
    // Returning values can also transfer ownership.
    main_return_values_and_scope();

    // Can also return multiple values
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn main_ownership_funcion() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn main_return_values_and_scope() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
