#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(u32, u32), // 宽度和高度
    Move { x: i32, y: i32 }, // x 和 y 坐标
    Echo(String), // 一个字符串消息
    ChangeColor(u8, u8, u8), // RGB 颜色值
    Quit, // 退出消息
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
