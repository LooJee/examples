fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count : {count}");
        let mut remaining = 10;

        loop {
            println!("remaining : {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("Ending count : {count}");

    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };

    println!("result : {result}");

    while_loop();
    for_loop();
    rev_for_loop();
}

fn while_loop() {
    let mut cnt = 3;

    while cnt > 0 {
        cnt -= 1;
    }

    println!("while loop end, get cnt : {cnt}");
}

fn for_loop() {
    let arr = [1, 2, 3, 4, 5];

    for ele in arr {
        print!("{ele},")
    }
    println!()
}

fn rev_for_loop() {
    // (1..4) 左为闭合区间，右为开放区间
    for ele in (1..4).rev() {
        print!("{ele},")
    }

    println!()
}
