fn main() {
    let m = Message::Write(String::from("Stuff"));
    m.call();
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("Yo the call worked!");
    }
}