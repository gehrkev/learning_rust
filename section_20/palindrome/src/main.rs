// No recursion, easy to read, additional time and memory overhead copying and reversing
fn palindrome_no_rec(vec: &Vec<i32>) -> bool {
    let mut vec2 = vec.to_vec();
    vec2.reverse();

    vec == &vec2
}

// Avoids copying and reversing, may cause stack overflow if the recursion depth is too large
fn palindrome(array: &Vec<i32>, start: usize, end: usize) -> bool {
    if start >= end {
        return true;
    }

    if array[start] == array[end] {
        return palindrome(array, start + 1, end - 1);
    } else {
        return false;
    }
}

fn main() {
    let mut vec = vec![1, 2, 3, 4];
    println!("{}", palindrome_no_rec(&vec));
    println!("{}", palindrome(&vec, 0, vec.len() - 1));

    vec = vec![1, 2, 3, 4, 3, 2, 1];
    println!("{}", palindrome_no_rec(&vec));
    println!("{}", palindrome(&vec, 0, vec.len() - 1));
}
