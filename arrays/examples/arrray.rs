fn main() {

    let arr: [u32; 4] = [65, 21, 4, 90];
    println!("arr: {:#?}", arr);
    println!("arr: {:?}", arr);

    let numbs:[i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];

    // First 3
    let ft: &[i32] = &numbs[..3];
    println!("The First three numbers are: {:?}", ft);
    
    // Last 3 indexes
    let lt: &[i32] = &numbs[7..];
    println!("This is the last 3 numbers: {:#?}", lt);
    
    // Middle 4 items
    let mid = &numbs[3..7];
    println!("The middle numbers are {:?}", mid);

    // Give all
    let all = &numbs[..];
    println!("The whole array is: {:?}", all);

}
