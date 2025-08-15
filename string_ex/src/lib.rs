#![allow(unused)]

fn main() {
    let msg: String = String::from("Hello Rust");
    let msg: String = "Hello Rust".to_string();

    let length: usize = msg.len();

    println!("The length is: {:?}", length) 
}
