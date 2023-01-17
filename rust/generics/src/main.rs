#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    the_struct_generic();

    the_fn_generic();

    the_mixup_generic();
}

fn the_struct_generic() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("both integer: {:?}", both_integer);
    println!("both float: {:?}", both_float);
    println!("integer and float: {:?}", integer_and_float);
}

fn the_fn_generic() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}, p.y = {}", p.x(), p.y());
}

fn the_mixup_generic() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point {
        x: "Hello world",
        y: 'c',
    };

    let p3 = p1.mixup(p2);

    println!("p3 = {:?}", p3);
}
