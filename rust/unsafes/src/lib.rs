use std::slice;

#[cfg(test)]
mod tests {
    use crate::split_at_mut;

    #[test]
    fn test_unsafe_pointer() {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is {}", *r1);
            println!("r2 is {}", *r2);

            *r2 = 10;

            assert_eq!(10, *r1);
            assert_eq!(10, *r2);
            assert_eq!(10, num);
        }
    }

    #[test]
    fn test_split_at_mut() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    #[test]
    fn test_extern() {
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    #[test]
    fn test_static_variable() {
        static mut COUNTER: u32 = 0;

        fn add_to_count(inc: u32) {
            unsafe { COUNTER += inc }
        }

        add_to_count(3);

        unsafe {
            assert_eq!(3, COUNTER);
        }
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
