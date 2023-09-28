fn fact(num: u32) -> u32 {
    // if num > 1 {
    //     return num * fact(num -1 )
    // } else {
    //     return 1
    // }
    match num {
        0 => 1, // That could be 0 | 1 => 1 but this is more mathematically accurate
        _ => num * fact(num - 1),
    }
}

fn main() {
    println!("{}", fact(0));
}
