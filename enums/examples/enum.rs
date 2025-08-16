#![allow(unused)]

#[derive(Debug)]
enum Command{
    Play,
    Pause,
    Stop,
    Skip(u32),
    Back(u32),
    Resize {width:u32, height: u32}
}

fn main() {
    let cmd: Command = Command::Play;
    let cmd: Command = Command::Back(30);
    let cmd: Command = Command::Resize {width: 100, height:50};

    println!("{:?}", cmd);
}