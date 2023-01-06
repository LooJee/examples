fn main() {
    parse_message(Message::Move(MoveMessage { x: 10, y: 12 }));
    parse_message(Message::Write(WriteMessage("hello world".to_string())));
    parse_message(Message::ChangeColor(ChangeColorMessage(0, 0, 0)));
    parse_message(Message::Quit(QuitMessage));
}

fn parse_message(msg: Message) {
    match msg {
        Message::Quit(msg) => msg.call(),
        Message::Move(msg) => msg.call(),
        Message::Write(msg) => msg.call(),
        Message::ChangeColor(msg) => msg.call(),
    }
}

enum Message {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage),
}

struct QuitMessage;

impl QuitMessage {
    fn call(&self) {
        println!("quit...");
    }
}

struct MoveMessage {
    x: i32,
    y: i32,
}

impl MoveMessage {
    fn call(&self) {
        println!("move to point ({}, {})", self.x, self.y);
    }
}

struct WriteMessage(String);

impl WriteMessage {
    fn call(&self) {
        println!("write message : {}", self.0);
    }
}

struct ChangeColorMessage(i32, i32, i32);

impl ChangeColorMessage {
    fn call(&self) {
        println!("change color to : ({}, {}, {})", self.0, self.1, self.2);
    }
}
