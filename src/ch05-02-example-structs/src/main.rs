fn main() {
    main1();
    main2();
    main3();
}

// Computing the area of a rectangle using basic types
// ..................................................
fn main1() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

// Refactor using tuples
// ....................
fn main2() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Refactor using structs
// .....................

// Deriving Debug trait to help printing information
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main3() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("rect1 is {:?}", rect1);

    // dbg! macro is very useful: prints the file and line number
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
