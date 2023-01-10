fn main() {
    let mut v = vec![1, 2, 3, 4];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);

    //使用下标获取元素
    let third = &v[2];

    //这里编译器会报错，因为 third 获取了 vector 元素的不可变引用，
    //如果这里 push 的时 vector 没有足够的空间，那么会分配新的空间，然后将老的数据拷贝到新的空间中
    //这时，third 指向的内存就被释放了，所以 rust 编译器会阻止这种行为。
    //v.push(9);

    println!("The third element is {}", third);

    output(v.get(2));

    range_vector(&v);

    for i in &mut v {
        *i += 50;
    }

    range_vector(&v);

    v.pop();
    range_vector(&v);

    enum_vector();
}

fn range_vector(v: &Vec<i32>) {
    for i in v {
        println!("{i}")
    }
}

fn output(v: Option<&i32>) {
    match v {
        Some(element) => println!("The element is {}", element),
        None => println!("There is no third element."),
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_vector() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.3),
        SpreadsheetCell::Text("hello world".to_string()),
    ];

    for ele in &row {
        match ele {
            SpreadsheetCell::Int(value) => println!("int value: {value}"),
            SpreadsheetCell::Float(value) => println!("float value: {value}"),
            SpreadsheetCell::Text(value) => println!("text value: {value}"),
        }
    }
}
