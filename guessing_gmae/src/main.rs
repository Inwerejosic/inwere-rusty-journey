use std::io;



fn main() {
    println!("Guess the number!");

    println!("Please input your guess, here: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to Read Line");

    println!("You guessed {}", guess)
}
