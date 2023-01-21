use std::fmt::Display;

use aggregator::{NewArticle, Tweet};

use crate::aggregator::Summary;

pub mod aggregator;

fn main() {
    let tweet = Tweet {
        username: String::from("hourse_ebboks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 now tweet: {}", tweet.summarize());

    let article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Chanpionship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&tweet);
    notify(&article);

    // 使用 impl trait 传递参数，允许两个参数的类型不同
    notify_multi_param(&tweet, &article);

    // 这里会报错，因为使用 trait bound 约束了两个参数的类型必须相同
    // notify_trait_bound(&tweet, &article);
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_multi_param(item1: &impl Summary, item2: &impl Summary) {
    println!("item1: {}, item2: {}", item1.summarize(), item2.summarize());
}

fn notify_trait_bound<T: Summary>(item1: &T, item2: &T) {
    println!("item1: {}, item2: {}", item1.summarize(), item2.summarize());
}

fn notify_multi_trait(item: &(impl Summary + Display)) {
    println!("item: {}", item);
}
