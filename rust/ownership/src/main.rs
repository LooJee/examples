fn main() {
    let s = String::from("hello world");
    takes_ownership(s);
    //    println!("{s}"); // s has moved into takes_ownership, here will be error

    let x = 5;
    make_copy(x);
    println!("{x}"); // i32 is a build-in type, it has implemented COPY trait, won't err here

    let s = gives_ownership();
    println!("{s}"); // s take ownership from gives_ownership
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn make_copy(some_integer: i32) {
    println!("{some_integer}")
}

fn gives_ownership() -> String {
    String::from("hello world") // move ownership out to the caller
}
