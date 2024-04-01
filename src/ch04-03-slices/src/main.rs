fn main() {

    // Slices
    // .....
    let mut s = String::from("hello world");
    let word1 = first_word_index(&s); // word will get the value 5
    println!("first_word {}", word1);

    let word2 = first_word_slice(&s);
    println!("first_word_slice {}", word2);

    s.clear(); // this empties the String, making it equal to ""

    // word1 still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word1 is now totally invalid!

    // If we try to access word2 we are going to get a compile error
    // since the slice is a reference to a continuous sequence of elements
    // because the string was cleared the slice is no longer valid
    // println!("first_word_slice {}", word2);

    // String Literals as Slices
    // ........................
    // The type of s here is &str: it’s a slice pointing to that specific point of the binary.
    // This is also why string literals are immutable; &str is an immutable reference.
    let s = "Hello, world!";
    println!("{}", s);

    // String Slices as Parameters
    // ..........................
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_slice_improved(&my_string[0..6]);
    let word = first_word_slice_improved(&my_string[..]);

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_slice_improved(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_slice_improved(&my_string_literal[0..6]);
    let word = first_word_slice_improved(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice_improved(my_string_literal);

    // Other slices
    // ...........
    let a = [1, 2, 3, 4, 5];

    // This slice has the type &[i32].
    // It works the same way as string slices do, by storing a reference to the first element and a length.
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// It allows us to use the same function on both &String values and &str values.
// If we have a string slice, we can pass that directly.
// If we have a String, we can pass a slice of the String or a reference to the String.
fn first_word_slice_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
