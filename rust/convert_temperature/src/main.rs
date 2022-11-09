use std::{io, num::ParseFloatError};

fn main() {
    loop {
        println!("Let's converting temperature:");
        println!("1. Convert Celsius to Fahrenheit.");
        println!("2. Convert Fahrenheit to Celsius.");
        println!("q. quit.");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("get choice failed");

        if choice.trim().cmp("q").is_eq() {
            break;
        }

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => handle_ctof(),
            2 => handle_ftoc(),
            _ => {
                println!("invalid choice");
                continue;
            }
        }

        println!();
    }
}

fn handle_ctof() {
    println!("please input celsius: ");

    let celsius: f32 = match read_line_f32() {
        Ok(num) => num,
        Err(e) => {
            println!("read line failed :{e}");
            return;
        }
    };

    let fahrenheit = ctof(celsius);

    println!("celsius: {celsius} => fahrenheit: {fahrenheit}")
}

fn ctof(c: f32) -> f32 {
    return c * 1.8 + 32.0;
}

fn handle_ftoc() {
    println!("please input fahrenheit: ");

    let fahrenheit: f32 = match read_line_f32() {
        Ok(num) => num,
        Err(e) => {
            println!("read line failed :{e}");
            return;
        }
    };

    let celsius = ftoc(fahrenheit);

    println!("fahrenheit: {fahrenheit} => celsius: {celsius}")
}

fn ftoc(f: f32) -> f32 {
    return (f - 32.0) / 1.8;
}

fn read_line_f32() -> Result<f32, ParseFloatError> {
    let mut data = String::new();
    io::stdin()
        .read_line(&mut data)
        .expect("read from stdin failed");

    let data = match data.trim().parse::<f32>() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };

    return Ok(data);
}
