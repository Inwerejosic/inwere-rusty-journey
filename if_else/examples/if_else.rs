#[allow(unused)]

fn main() {
    let mba:i32 = 10;

    if mba % 2 == 0 {
        println!("{mba} is even!");
    } else {
        println!("{mba} is odd!");
    }

    let y: i32 = if mba > 0 {
        1
    } else if mba < 0 {
        -1
    } else {
        0
    };

    println!("{}", y);
}