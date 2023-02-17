#[cfg(test)]
mod tests {
    use std::ops::Add;

    #[test]
    fn test_point() {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, rhs: Point) -> Point {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        assert_eq!(
            Point { x: 1, y: 1 } + Point { x: 2, y: 2 },
            Point { x: 3, y: 3 }
        )
    }

    #[test]
    fn test_meter() {
        struct Millimeters(u32);
        struct Meters(u32);

        impl Add<Meters> for Millimeters {
            type Output = Millimeters;

            fn add(self, rhs: Meters) -> Millimeters {
                Millimeters(self.0 + (rhs.0 * 1000))
            }
        }

        assert_eq!((Millimeters(1000) + Meters(1)).0, Millimeters(2000).0);
    }
}
