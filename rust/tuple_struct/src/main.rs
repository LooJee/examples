fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black color : ({}, {}, {})", black.0, black.1, black.2);
    println!("origin point : ({}, {}, {})", origin.0, origin.1, origin.2);
}

struct Point(i32, i32, i32);
struct Color(i32, i32, i32);
