fn selection_sort(vector: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..vector.len() - 1 {
        let mut smallest = i;
        for j in (i + 1)..vector.len() {
            if vector[j] < vector[smallest] {
                smallest = j;
            }
        }
        vector.swap(smallest, i);
    }
    vector.to_vec()
}

fn bubble_sort(vector: &mut Vec<i32>) -> Vec<i32> {
    let mut sorted = true;
    for _ in 1..=vector.len() - 1 {
        sorted = true;
        for j in 0..=vector.len() - 2 {
            if vector[j] > vector[j + 1] {
                vector.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
    vector.to_vec()
}

fn merge_sort(vector: &mut [i32]) -> Vec<i32> {
    if vector.len() > 1 {
        let mid = vector.len() / 2;
        merge_sort(&mut vector[..mid]);
        merge_sort(&mut vector[mid..]);
        merge(vector, mid);
    }
    vector.to_vec()
}

// Helpíng function
fn merge(vector: &mut [i32], mid: usize) {
    let left = vector[..mid].to_vec();
    let right = vector[mid..].to_vec();

    let mut l = 0;
    let mut r = 0;

    for val in vector {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *val = left[l];
            l += 1;
        } else {
            *val = right[r];
            r += 1;
        }
    }
}

fn quick_sort(vector: &mut [i32], start: usize, end: usize) -> Vec<i32> {
    if start < end {
        let part = partition(vector, start, end);
        quick_sort(vector, start, part - 1);
        quick_sort(vector, part + 1, end);
    }
    vector.to_vec()
}

// Helping function
fn partition(vector: &mut [i32], start: usize, end: usize) -> usize {
    let mut i = start;
    let pivot = end;


    for j in start..=end - 1 {
        if vector[j] < vector[pivot] {
            vector.swap(i, j);
            i += 1;
        }
    }

    vector.swap(i, pivot);
    i
}

fn main() {
    let mut vec = vec![4, 3, 1, 2];
    println!("Before selection sorting: {:?}", vec);
    selection_sort(&mut vec);
    println!("After selection sorting: {:?}\n", vec);

    vec = vec![5, 4, 1, 3, 2];
    println!("Before bubble sorting: {:?}", vec);
    bubble_sort(&mut vec);
    println!("After bubble sorting: {:?}\n", vec);

    vec = vec![4, 7, 3, 5, 1, 2];
    println!("Before merge sorting: {:?}", vec);
    merge_sort(&mut vec);
    println!("After merge sorting: {:?}\n", vec);

    let mut vec = vec![8, 5, 1, 2, 7, 3, 4];
    let len = vec.len() - 1;
    println!("Before quick sorting: {:?}", vec);
    quick_sort(&mut vec, 0, len);
    println!("After quick sorting: {:?}", vec);
}

