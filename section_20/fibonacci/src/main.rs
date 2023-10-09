fn fib(num: u32) -> u32 {
    match num {
        0 => 0,
        1 => 1,
        _ => {
            let n1 = fib(num - 1);
            let n2 = fib(num - 2);
            n1 + n2
        }
    }
}

fn main() {
    println!("{}", fib(15));
}
