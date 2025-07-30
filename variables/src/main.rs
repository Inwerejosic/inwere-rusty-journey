fn main() {
    let x = 21;

    let x = x + 32;

    {
        let x = x + 24;
        println!("The value of X is:, {x}");
    }

    println!("The value of X is:, {x}");
}