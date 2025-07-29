use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 1001);


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
    // println!("The secretnumer is: {}", secret_number);

    
    match azrs.cmp(&secret_number) {
        Ordering::Less => println!("Too Small oh"),
        Ordering::Greater => println!("Too big oh"),
        Ordering::Equal => println!("You are spot on!")
    }
}