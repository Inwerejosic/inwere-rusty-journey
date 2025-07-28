use std::io;

fn main() {
    // Declare a mutable String to store input
    let mut azrs = String::new();

    // Read input from stdin
    io::stdin()
        .read_line(&mut azrs)
        .expect("Failed to read line");

    // Trim the input to remove newline and parse to i32
    let azrs: i32 = azrs
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Compare the parsed number
    if azrs >= 2 {
        println!("I love Learning");
    } else {
        println!("Out of here");
    }
}