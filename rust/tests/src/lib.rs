use core::panic;

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn get_area(&self) -> u32 {
        self.height * self.width
    }

    fn check_panic(is_panic: bool) {
        if is_panic {
            panic!("let's panic here");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn check_area() {
        let rect = Rectangle {
            width: 2,
            height: 2,
        };

        //断言通过
        assert_eq!(4, rect.get_area());
        //断言通过
        assert_ne!(5, rect.get_area());
        //断言失败
        assert_eq!(5, rect.get_area());
    }

    #[test]
    fn custom_check_can_hold() {
        let larger = Rectangle {
            width: 5,
            height: 2,
        };

        let smaller = Rectangle {
            width: 4,
            height: 3,
        };

        assert!(
            larger.can_hold(&smaller),
            "larger: {:?} cannot hold smaller: {:?}",
            larger,
            smaller
        )
    }

    #[test]
    #[should_panic]
    fn get_panic() {
        Rectangle::check_panic(true);
    }

    #[test]
    #[should_panic]
    fn not_panic() {
        Rectangle::check_panic(false);
    }
}
