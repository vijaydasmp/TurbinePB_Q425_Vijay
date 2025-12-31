#[derive(Debug)]
enum Message {
    Quit,
    Move,
    Echo,
    ChangeColor,
    Resize,}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
