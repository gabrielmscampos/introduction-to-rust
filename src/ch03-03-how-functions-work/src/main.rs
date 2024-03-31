fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // This is an expression
    // That value gets bound to y as part of the let statement.
    // Note that the x + 1 line doesn’t have a semicolon at the end,
    // which is unlike most of the lines you’ve seen so far.
    // Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression,
    // you turn it into a statement, and it will then not return a value.
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions can return values to the code that calls them.
// We don’t name return values, but we must declare their type after an arrow (->).
// the body of the function is a lonely 5 with no semicolon because it’s an expression whose value we want to return.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // If we add a semicolon to the end of an expression, we turn it into a statement, and it will then not return a value.
}
