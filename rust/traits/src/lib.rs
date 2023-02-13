mod post;

#[cfg(test)]
mod tests {
    use crate::post::Post;
    use crate::{Button, Screen, SelectBox};

    #[test]
    fn test_screen() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("Ok"),
                }),
            ],
        };

        screen.run();
    }

    #[test]
    fn test_post() {
        let mut post = Post::new();

        let text = "hello world from Rust";

        post.add_text(text);

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!(text, post.content());
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw a button: {:?}", self);
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw a select box:{:?}", self);
    }
}
