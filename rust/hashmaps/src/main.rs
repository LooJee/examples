use std::collections::HashMap;

fn main() {
    //通过new函数创建一个HashMap
    let scores = new();
    if let Some(score) = scores.get("Blue") {
        println!("socre of team Blue is: {}", score);
    }

    for (key, value) in &scores {
        println!("score of team: {} is: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    with_entry(&mut scores);
    println!("scores after insert with entry: {:?}", scores);

    //通过迭代器、zip&&collect来新建一个HashMap
    new_by_zip();

    //通过 entry 方法和 or_insert 来统计字符串中单词的个数
    count_word("hello world wonderful world");
}

fn new() -> HashMap<String, i32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    println!("new by new function: {:?}", scores);

    scores
}

fn new_by_zip() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("new hash map from zip iter and collect : {:?}", scores);
}

fn with_entry(scores: &mut HashMap<String, i32>) {
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
}

fn count_word(text: &str) {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count += 1
    }

    println!("count word: {:?}", map);
}
