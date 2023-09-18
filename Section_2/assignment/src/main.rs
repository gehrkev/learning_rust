// use std::ops::Add; // Question 3

fn main() {
    // 1
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;
    println!("{}", ans);
    // 2
    let mut vector = vec![2,4,6,8,10];
    println!("{}", vector[0]);
    println!("{}", vector[1]);
    println!("{}", vector[2]);
    println!("{}", vector[3]);
    println!("{}", vector[4]);
    println!("{:?}", vector); // instructor solution
    vector.pop();
    vector.push(12);
    println!("{}", vector[0]);
    println!("{}", vector[1]);
    println!("{}", vector[2]);
    println!("{}", vector[3]);
    println!("{}", vector[4]);
    println!("{:?}", vector); // instructor solution
    // 3
    let string = "Hello".to_string();
    println!("{}", string);
    println!("{}", concat_string(string.clone()));
    //4
    control_flow(1);
    control_flow(51);
    control_flow(0);
    control_flow(42);
}

// 3
fn concat_string(some_string: String) -> String {
    // my solution
    // let other_string = " World".to_string();
    // some_string.clone().add(&other_string)
    some_string + " World!" //instructor solution
}

// 4
fn control_flow(arg: u8) {
    if arg == 1 {
        println!("the value is one")
    } else if arg > 50 {
        println!("the value is greater than 50")
    } else if arg < 25 {
        println!("the value is less than 25")
    } else {
        println!("the value is greater than 25 but less than 50")
    }
}
