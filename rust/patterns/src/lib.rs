#[cfg(test)]
mod tests {

    #[test]
    fn test_match() {
        let data: Option<i32> = None;

        let data = match data {
            None => None,
            Some(i) => Some(i + 1),
        };

        assert_eq!(None, data);

        let data = Some(1);

        let data = match data {
            None => None,
            Some(i) => Some(i + 1),
        };

        assert_eq!(Some(2), data);
    }

    #[test]
    fn test_if_let() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    #[test]
    fn test_while_let() {
        let mut stack = vec![1, 2, 3];

        while let Some(pop) = stack.pop() {
            println!("{pop}");
        }
    }

    #[test]
    fn test_for() {
        let v = vec![1, 2, 3];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    #[test]
    fn test_match_more() {
        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            3..=6 => println!("three through six"),
            _ => println!("something else"),
        }
    }

    #[test]
    fn test_match_guard() {
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }
    }
}
