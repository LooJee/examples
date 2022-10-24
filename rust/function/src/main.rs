fn main() {
    let b = plus_one(5);

    println!("5 + 1 = {b}");
}

// x 是函数形参，类型为 i32
// -> i32 表示函数返回 i32 类型的值
fn plus_one(x: i32) -> i32 {
    // x + 1 是函数最后一个表达式，隐式作为函数的返回值
    x + 1
}
