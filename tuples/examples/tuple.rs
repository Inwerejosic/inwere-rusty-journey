#![allow(unused)]

fn main() {
    let t:(char, bool, u32) = ('X', false, 34);
    println!("{}, {}, {}", t.0, t.1, t.2);

    // Adding a empty tuple - unit type
    let t = ();

    // Nested Tuple
    let nested = (('a', 1.43, false), (true, 1i64, -1i32), ());
    let (x, y, z) = nested;
    println!("x = {:#?}, y = {:#?}, z = {:?}", x, y, z);
    println!("nested.0.1: {}", (nested.0).1);

    // Destructuring a tuple
    let t:(char, bool, u32) = ('X', false, 36);
    let (a, b, c) = t;
    println!("a = {}, b = {}, and c = {}", a, b, c);
}