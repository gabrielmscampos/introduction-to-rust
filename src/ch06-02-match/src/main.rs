enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // This seems very similar to a conditional expression used with if,
    // but there’s a big difference: with if, the condition needs to evaluate to a
    // Boolean value, but here it can be any type

    // When the match expression executes, it compares the resultant
    // value against the pattern of each arm, in order. If a pattern matches the value,
    // the code associated with that pattern is executed

    // Matches must cover all possibilities of the enum
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("value: {}", value);

    // Catch-all pattern
    // ................
    // Using enums, we can also take special actions for a few particular values,
    // but for all other values take one default action
    let dice_roll = 9;

    // This code compiles, even though we haven’t listed all the possible values a u8 can have,
    // because the last pattern will match all values not specifically listed
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Rust also has a pattern we can use when we want a catch-all but don’t want to use the
    // value in the catch-all pattern: _ is a special pattern that matches any value and does not bind to that value
    // This example also meets the exhaustiveness requirement because we’re explicitly ignoring all other values in the last arm;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}
