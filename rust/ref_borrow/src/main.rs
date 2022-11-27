fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 获取 s1 的引用的过程被称为借用

    println!("len of s1 : {len}");

    // s1 原来不是可修改的变量，所以需要重新定义一个可修改的变量
    let mut s1 = s1;
    change(&mut s1);

    println!("changed s1 : {s1}");

    let mut s1 = s1;
    let s2 = &s1;
    let s3 = &mut s1; // 一个变量不能同时拥有可变引用和不可变引用

    println!("{s2}");

    let mut s1 = s1;
    let s2 = &mut s1;
    let s3 = &mut s1; // 一个变量不能有多个可变引用，会导致数据竞争
}

// 该函数的参数是一个引用，它不会获取调用者参数的所有权
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 引用的值默认是不可以被修改的，只有指定参数为 mut 时，才可以修改引用的值
fn change(s: &mut String) {
    s.push_str(", world")
}

fn dangle() -> &String {
    let s = String::from("hello");

    //这里不能将 s 的引用返回，因为 s 在离开函数作用域时将会被释放，除非指定函数返回值的生命周期
    &s
}
