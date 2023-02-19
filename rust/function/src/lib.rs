#[cfg(test)]
mod tests {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    #[test]
    fn test_fn() {
        let answer = do_twice(add_one, 5);

        assert_eq!(12, answer);
    }

    #[test]
    fn test_return_closure() {
        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }

        let closure = returns_closure();
        assert_eq!(11, closure(10));
    }
}
