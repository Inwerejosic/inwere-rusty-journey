#![allow(unused)]

// Scalar types represents a single value
fn main() {
    // Signed Integers
    // Range = (2^(n-1)) to 2^(n-1) - 1
    let i0:i8 = -1;
    let i1:i16 = 2;
    let i2:i32 = 3;
    let i3:i64 = -4;
    let i4:i128 = 5;
    // Depends on Computer Architecture
    let i5:isize = -6;

    // Unsigned Integers
    // 0 to 2^n-1
    let u0:u8 = 1;
    let u1:u16 = 2;
    let u2:u32 = 3;
    let u3:u64 = 4;
    let u4:u128 = 5;
    // Depends on computer Architecture
    let u5:usize =6;

    // Type Conversion
    let i:i32 = -1;
    let u:u32 = i as u32;
    println!("i32: {} to u32: {}", i, u);


    // Min and Max
    let max = i32::MAX;
    let min = u32::MIN;
    println!("Maximum is {}", max);
    println!("Minimum is {}", min);
}