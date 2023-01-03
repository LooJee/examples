fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 20,
    };

    println!("we get rectangle : {:?}", rect1);
    println!("area of rect1 is : {}", rect1.area());
    println!("rect1 has nonezero width : {}", rect1.width());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(20);
    println!("Get square : {:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}
