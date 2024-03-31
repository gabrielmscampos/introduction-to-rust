fn main() {
    // Conditionals
    // -----------
    let number = 3;

    // Conditions in if blocks must always return a boolean value
    // Else is optional
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Handling multiple conditions
    // ---------------------------
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    // --------------------------
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Repeating code with loop
    // -----------------------
    // The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
    let mut counter: u8 = 0;
    loop {
        println!("again!");
        if counter == 2 {
            break;  // Break out of the loop
        }
        counter += 1;
    }

    // One of the uses of a loop is to retry an operation you know might fail,
    // such as checking whether a thread has completed its job. You might also
    // need to pass the result of that operation out of the loop to the rest of your code
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Loop labels to disambiguate loops
    // --------------------------------
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // While is a conditional loop
    // --------------------------
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // For loops trough a collection
    // ----------------------------
    // Example using while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // Example using for
    for element in a {
        println!("the value is: {element}");
    }

    // Instead of while to countdown we can use for with range:
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
