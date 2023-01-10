fn main() {
    let data = "initial content";

    let s = data.to_string();
    println!("get string: {}", s);

    let s = "initial content".to_string();
    println!("get string: {}", s);

    let s = String::from("initial contant");
    println!("initial string by String::from: {}", s);

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    // push_str 获取的是字符的引用
    println!("string after push_str: {}, s2 is: {}", s, s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("string after push: {}", s);

    //使用 + 操作符拼接字符串
    add_str();

    //使用 format! 宏拼接字符串
    format_str();

    utf8();
}

fn add_str() {
    let s1 = String::from("Hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    //  println!("s1: {}", s1); //这里会报错，因为s1在 + 操作中所有权已经被获取了。
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}

fn format_str() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format string: {}", s);
}

fn utf8() {
    let s = "你好";
    println!("len of {}: {}", s, s.len());
    //let c = s[0]; // 这里会报错。字符串无法下标索引。

    //will panic here
    println!("{}", &s[0..1]);

    //使用 chars 来遍历 Unicode 标量值
    println!("chars in string:");
    for ele in s.chars() {
        println!("{}", ele);
    }

    //使用 bytes 来遍历原始字节
    println!("bytes in string:");
    for ele in s.bytes() {
        println!("{}", ele)
    }
}
