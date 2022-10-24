fn main() {
    let (a, b, c) = get_tuple();

    println!("a: {a}, b: {b}, c: {c}");

    let tup = get_tuple();

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    println!("a: {a}, b: {b}, c: {c}");
}

fn get_tuple() -> (i32, i32, i32) {
    (1, 2, 3)
}
