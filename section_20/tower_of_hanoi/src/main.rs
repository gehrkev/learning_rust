fn hanoi(n: i32) -> i32 {
    if n == 0 {
        return 0
    }
    return hanoi(n-1) + 1 + hanoi(n-1)
}

fn main() {
    println!("{}", hanoi(0));
    println!("{}", hanoi(1));
    println!("{}", hanoi(2));
    println!("{}", hanoi(3));
    println!("{}", hanoi(4));
}
