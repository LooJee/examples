fn main() {
    let my_string = String::from("Hello world");

    let word = first_word(&my_string);

    println!("first word : {word}");
}

// &str是字符串切片
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
