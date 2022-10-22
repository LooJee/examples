use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // 1..=100 是一个范围表达式

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // read_line 返回 Result 类型的值，Result 是一种枚举类型，它的成员有 Ok 和 Err 两种。
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guess : {guess}");

        // match 表达式由分支（arms）构成，每个分支包含一个模式
        // 以下的 Ordering::Equal, Ordering::Less, Ordering::Greater 代表三个分支。
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
