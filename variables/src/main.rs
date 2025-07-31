fn main() {
    let x = 21;

    let x = x + 32;

    {
        //  The inner expression
        let x = x + 24;
        println!("The value of X is:, {x}");
    }

    println!("The value of X is:, {x}");

    // Calling a function inside a function
    print_labled_measurement(6, 'm');

    fn print_labled_measurement(value:i32, unit_label:char){
        println!("The measurement is: {value}{unit_label}");
    }
}