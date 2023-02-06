use std::{cell::RefCell, ops::Deref, rc::Rc};

mod messenger;
mod tree;

#[cfg(test)]
mod tests {
    use crate::messenger::{LimitTracker, Messenger};

    use super::*;
    use tree::Node;
    use InnerList::{Next, Null};
    use List::{Cons, Nil};

    #[test]
    fn test_deref() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_box() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_mybox() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_rc() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

        let _b = Rc::new(Cons(3, Rc::clone(&a)));
        let _c = Rc::new(Cons(4, Rc::clone(&a)));

        assert_eq!(3, Rc::strong_count(&a));

        drop(_b);
        assert_eq!(2, Rc::strong_count(&a));

        drop(_c);
        assert_eq!(1, Rc::strong_count(&a));
    }

    /* RefCell 和 Rc 结合使用 */

    #[test]
    fn test_rc_refcel() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Next(Rc::clone(&value), Rc::new(Null)));

        let b = Next(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Next(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    /* RefCell 的应用 */
    struct MockMessenger {
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.send_messages.borrow_mut().push(String::from(msg))
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();

        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
    }

    #[test]
    fn test_weak_tree() {
        let output_count = |name: &str, node: &Rc<Node>| {
            println!(
                "{name} strong = {}, weak = {}",
                Rc::strong_count(&node),
                Rc::weak_count(&node)
            )
        };

        let leaf = Node::new(3);

        output_count("leaf", &leaf);

        {
            let branch = Node::new(5);

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            output_count("branch", &branch);
            output_count("leaf", &leaf);
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        output_count("leaf", &leaf);
    }
}

#[derive(Debug)]
enum InnerList {
    Next(Rc<RefCell<i32>>, Rc<InnerList>),
    Null,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop me");
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
