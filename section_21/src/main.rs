fn selection_sort(vector: &mut Vec<i8>) -> Vec<i8> {
    for i in 0..vector.len() - 1 {
        let mut smallest = i;
        for j in (i+1)..vector.len() {
            if vector[j] < vector[smallest] {
                smallest = j;
            }
        }
        vector.swap(smallest, i);
    }
    vector.to_vec()
}

fn bubble_sort(vector: &mut Vec<i8>) -> Vec<i8> {
    let mut sorted = true;
    for _ in 1..=vector.len() - 1 {
        sorted = true;
        for j in 0..=vector.len() -2 {
            if vector[j] > vector[j+1] {
                vector.swap(j, j+1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
    vector.to_vec()
}

fn merge_sort(vector: &mut Vec<i8>) -> Vec<i8> {
    //todo
    vector.to_vec()
}

fn main() {
    let mut vec = vec![4, 3, 1, 2];
    println!("Before selection sorting: {:?}", vec);
    selection_sort(&mut vec);
    println!("After selection sorting: {:?}", vec);

    vec = vec![5, 4, 1, 3, 2];
    println!("Before bubble sorting: {:?}", vec);
    bubble_sort(&mut vec);
    println!("After bubble sorting: {:?}", vec);

    vec = vec![4, 7, 3, 5, 1, 2];
    println!("Before merge sorting: {:?}", vec);
    merge_sort(&mut vec);
    println!("After merge sorting: {:?}", vec);
}

