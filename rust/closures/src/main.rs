use std::{fmt::Debug, thread};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn inventory() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn add_fn() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;

    add_one_v1(3);
    add_one_v2(3);
    add_one_v3(3);
}

fn main() {
    inventory();

    add_fn();

    closure_catch_variable();
}

fn closure_catch_variable() {
    let list = vec![1, 2, 3];

    // 这里的闭包根据 println! 的参数，得知只需要获取引用就可以了
    let only_borrow = || println!("From closure: {:?}", list);

    only_borrow();

    //因为 only_borrow 获取的是 list 的引用，所以这里还可以使用 &list
    output(&list);

    /*--------获取可变引用----- */
    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrow_mutably = || list.push(4);

        // 这段代码会报错，因为 list是一个可变借用，在使用之前无法借用不可变借用
        // println!("Before calling closure: {:?}", list);
        borrow_mutably();
        println!("After calling closure: {:?}", list);
    }
    /* 移动变量所有权到闭包中 */
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // 这里需要使用 move 移动所有权是因为线程的执行顺序是不确定的，主要还是 list 生命周期的问题。
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();

        // 这段代码会报错，因为 list 的所有权已经被移走了，这里无法再使用 list
        // println!("After calling thread::spawn: {:?}", list);
    }
}

fn output<T: Debug>(v: &T) {
    println!("From output: {:?}", v);
}
