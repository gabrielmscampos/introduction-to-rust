fn main() {
    // == Mutability ==
    // If this variable was immutable, it would not compile
    // Returning the following error when trying ot reassign x value:
    // ^^^^^ cannot assign twice to immutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // == Constants ==
    // Constants aren’t just immutable by default—they’re always immutable
    // The last difference is that constants may be set only to a constant expression,
    // not the result of a value that could only be computed at runtime.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // == Shadowing ==
    // Rustaceans say that the first variable is shadowed by the second,
    // which means that the second variable is what the compiler will see when
    // you use the name of the variable. In effect, the second variable overshadows the first,
    // taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
    // We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    // Shadowing is different from marking a variable as mut because we’ll get a compile-time error
    // if we accidentally try to reassign to this variable without using the let keyword. By using let,
    // we can perform a few transformations on a value but have the variable be immutable after
    // those transformations have been completed.

    // The other difference between mut and shadowing is that because we’re effectively
    // creating a new variable when we use the let keyword again, we can change the type
    // of the value but reuse the same name. For example, say our program asks a user to show
    // how many spaces they want between some text by inputting space characters,
    // and then we want to store that input as a number:
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // This would have been a compile-time error because we can’t mutate a variable’s type
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
