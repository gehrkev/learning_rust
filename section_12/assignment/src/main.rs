fn main() {
    let vec: Vec<i32> = vec![1, 3, 5, 7, 9];
    let times_ten: Vec<i32> = vec.iter().map(|x| x * 10).collect();
    println!("{:?}", times_ten);
}
