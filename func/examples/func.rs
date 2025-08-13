#![allow(unused)]
use func::div;

fn add_with_return(x:u32, y:u32) -> u32 {
    // This is an implementation for additon with a return
    return x + y;
}

fn add(x:u32, y:u32) -> u32 {
    // This implementation for implicite returns
    x + y
}

fn main() {
    let x:u32 = 60;
    let y:u32 = 5;
    let z:u32 = add(x, y);
    let quotient = div(x, y);
    println!("The Sum of {} + {} = {}", x, y, z);
    println!("The Sum of {} + {} = {}", x, y,  quotient);
}