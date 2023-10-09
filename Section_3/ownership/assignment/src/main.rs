fn main() {
    let mut v = vec![1, 3, 5, 7];
    println!("{}", function(&v));
    // println!("{}",function(v.clone()));
    v.push(15);
    println!("{:?}", v);

    let mut val1 = 1;
    add_two(val1);
    println!("{}", val1);
    println!("{}", add_two(val1));
}

fn function(val: &Vec<i32>) -> bool {
    // fn function(val: Vec<i32>) -> bool {
    if val[0] == 1 {
        true
    } else {
        false
    }
}

fn add_two(val: i8) -> i8 {
    let return_val: i8 = val + 2;
    return_val
}
