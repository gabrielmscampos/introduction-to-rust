fn main() {

    // References
    // .........
    let s1 = String::from("hello");

    // Instead of returning the same string we moved to the function
    // we can provide a reference to the string value
    // A reference is like a pointer in that it’s an address we can follow
    // to access the data stored at that address; that data is owned by some other variable.
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable References
    // .................
    let mut s = String::from("hello");
    change(&mut s);
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    // If we have a mutable reference to a value we can have no other references to it.
    // let r1 = &mut s;
    // let r2 = &mut s;

    // In another scope this is possible
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("{}", r2);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References
    // ..................
    // The compiler guarantees that references will never be dangling references:
    // if you have a reference to some data, the compiler will ensure that the data
    // will not go out of scope before the reference to the data does.
    //
    // fn dangle() -> &String { // dangle returns a reference to a String
    //     let s = String::from("hello"); // s is a new String
    //     &s // we return a reference to the String, s
    // } // Here, s goes out of scope, and is dropped. Its memory goes away.
    //   // Danger!
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// If we try to modify a reference we will get an error
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// We can modify a reference if it is mutable
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

