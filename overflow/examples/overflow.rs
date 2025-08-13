#![allow(unused)]

fn main() {
    let mut x = u32::MAX;
    x += 1;

    println!("u32 Max: {},  x: {}", u32::MAX, x);

    // u32::checked_add - Return None on overflow
    let x = u32::checked_add(5, 1);
    println!("Checked_add: {:?}", x);
    // u32::wrapping_add - Implicitly allow overflow


}
