#![allow(unused)]

#[derive(Debug, PartialEq)]
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

    println!("{:#?}", cmd);

    let cmd1: Command = Command::Pause;
    let cmd2: Command = Command::Skip(34);

    println!("{}", cmd2 == cmd2);
    println!("{}", cmd2 == cmd1);

    // Option<T> = Some(T) | None
    let x: Option<i32> = Some(-1);
    let X: Option<i32> = None;
    println!("{:?}", x );

    // Result<T, E> = Ok(T) | Error(E)
    let res: Result<i32, &str> = Ok(100);
    let res: Result<i32, &str> = Err("Error 💀");
    println!("{:?}", res);

}