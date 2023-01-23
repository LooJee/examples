fn main() {
    println!("longest is : {}", longest("hello", "world"));

    let str = "hello world";
    {
        let data = Lifetime { part: str };
        println!("{:?}", data);

        println!("{}", data.get_part());
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Lifetime<'a> {
    part: &'a str,
}

impl<'a> Lifetime<'a> {
    fn get_part(&self) -> &str {
        self.part
    }
}
