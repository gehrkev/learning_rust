fn sum_triangle(vec: &Vec<i8>) {
    if vec.len() < 1 {
        return;
    }

    let mut tmp: Vec<i8> = Vec::new();
    for i in 0..vec.len() - 1 {
        let a = vec[i];
        let b = vec[i + 1];
        let c = a + b;
        tmp.push(c);
    }

    sum_triangle(&tmp);
    println!("{:?}", vec);
}

// Guide solution
fn triangle(arr: &mut Vec<i8>, size: usize) {
    if size < 1 {
        return;
    }

    let mut tmp: Vec<i8> = Vec::new();

    for i in 0..size - 1 {
        let x = arr[i] + arr[i + 1];
        tmp.push(x);
    }

    triangle(&mut tmp, size - 1);
    println!("{:?}", arr);
}

fn main() {
    let mut a = vec![1, 2, 3, 4, 5];
    sum_triangle(&a);
    // Guide solution
    let mut b = vec![1, 2, 3, 4, 5];
    let size = b.len();
    triangle(&mut b, size);
}
