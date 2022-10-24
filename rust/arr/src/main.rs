fn main() {
    let a = [1, 2, 3, 4, 5];

    for ele in a {
        print!("{ele} ");
    }
    println!();

    // 定义数组 a 为有 5 个元素的 i32 数组。
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for ele in a {
        print!("{ele} ");
    }
    println!();

    // 元素个数也是数组类型的一部分，下面这段代码会报类型不匹配
    // let a: [i32; 4] = [1, 2, 3, 4, 5];

    // 下面这段代码声明了一个数组 a， 并给它初始化为长度为 5，每个元素都为 3 的数组
    let a = [3; 5];
    for ele in a {
        print!("{ele} ");
    }
    println!();
}
