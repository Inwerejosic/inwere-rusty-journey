// // Importing the necessary tools
// use std::io;

// fn main() {
//     println!("This is a guessing game!");

//     println!("Please input your guess: ");

//     // let mut guess = String::new();
//     let mut ekele = String::new();
    
//     // io::stdin()
//     // .read_line(&mut guess).
//     // expect("Failed to read line");

//     io::stdin()
//     .read_line(&mut ekele).
//     expect("Line Read successfuly!");

//     // println!("You guessed: {}", guess)
//     println!("You guessed: {}", ekele)
// }






































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
        println!("Out");
    }
}