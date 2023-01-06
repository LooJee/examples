fn main() {
    output(Some(10));
    output(None);

    let x = Some(2);
    assert_eq!(x.is_none(), false);

    let x: Option<i32> = None;
    assert_eq!(x.is_none(), true);

    output(add(Some(10)));
    output(add(None));
}

fn output(o: Option<i32>) {
    if let Some(num) = o {
        println!("get number : {num}");
    } else {
        println!("get none here");
    }
}

fn add(o: Option<i32>) -> Option<i32> {
    if let Some(num) = o {
        Some(num + 1)
    } else {
        None
    }
}
