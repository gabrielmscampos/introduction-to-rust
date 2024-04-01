struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("active: {} user: {} email: {} sign_in_count: {}", user1.active, user1.username, user1.email, user1.sign_in_count);

    // We can instantiate mutable struct to change values
    // .................................................
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("active: {} user: {} email: {} sign_in_count: {}", user1.active, user1.username, user1.email, user1.sign_in_count);

    // Using build_user function with init shorthand
    // ............................................
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    println!("active: {} user: {} email: {} sign_in_count: {}", user1.active, user1.username, user1.email, user1.sign_in_count);

    // Creating Instances from Other Instances with Struct Update Syntax
    // ................................................................
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("active: {} user: {} email: {} sign_in_count: {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    // We can achieve the same effect using struct update syntax
    // ........................................................
    let user2 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    println!("active: {} user: {} email: {} sign_in_count: {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    // Using Tuple Structs Without Named Fields to Create Different Types
    // .................................................................
    // Each struct you define is its own type, even though the fields within the struct might have the same types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: ({} {} {}) origin: ({} {} {})", black.0, black.1, black.2, origin.0, origin.1, origin.2);

    // Unit-Like Structs Without Any Fields
    // ...................................
    // Unit-like structs can be useful when you need to implement a trait on
    // some type but don’t have any data that you want to store in the type itself.
    let subject = AlwaysEqual;
}

// Field init shorthand
// no need to specify fields names while assigning values
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
