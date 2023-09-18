fn main() {
    // print_phrase("print my arg");
    println!("{}", gcd(20, 5));
    println!("{}", multiple_return_values(false));
}

// fn print_phrase(phrase: &str){
//     println!("{}", phrase);
// }

fn gcd(mut a: u64, mut b: u64) -> u64 { //greatest common denominator
    while a != 0{
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b //returns b without ';'
}

fn multiple_return_values(flag: bool) -> bool {
    if flag == true{
        true // notice there's no ';' - returns it
    } else {
        false // same as above
    }
}
