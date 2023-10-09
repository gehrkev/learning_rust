// use rand::seq::SliceRandom;
// use rand::thread_rng;
// use std::collections::BinaryHeap;
// use std::collections::{hash_map, HashMap};

use std::collections::{hash_set, HashSet};

pub fn main() {
    // let mut nums: Vec<i32> = vec![];
    //
    // nums.push(1);
    // nums.push(2);
    // nums.push(3);
    //
    // let pop = nums.pop(); // -> Option<T>, return None or Some(T)
    //
    // println!("{:?}",pop);
    //
    // let two = nums[1]; //copy
    // //&nums[1], creates a ref if copy is not available
    //
    // println!("{}", two);
    //
    // let one = nums.first(); //return an Option<T>, none if vec empty or Some(T) if not empty
    // println!("{:?}", one);
    //
    // // .last
    // // .first_mut and .last_mut, will borrow mut refs
    //
    // println!("{}", nums.len()); //returns value of length
    // println!("{}", nums.is_empty()); //bool
    //
    // nums.insert(0, 10);
    // nums.insert(3, 12);
    // nums.insert(2, 25);
    //
    // nums.remove(3);
    //
    // nums.sort();
    // println!("{:?}", nums);
    //
    // nums.reverse();
    // println!("{:?}", nums);
    //
    // nums.shuffle(&mut thread_rng());
    // println!("{:?}", nums);

    // let mut bheap = BinaryHeap::new();
    //
    // bheap.push(1);
    // bheap.push(18);
    // bheap.push(20);
    // bheap.push(5);
    //
    // bheap.pop(); //removes value at the front
    //
    // println!("{:?}", bheap);
    //
    // println!("{:?}", bheap.peek()); // -> Option<T>, returns none or some(t)

    // let mut hm = HashMap::new();
    //
    // hm.insert(1,1);
    // hm.insert(5,2);
    // hm.insert(30,3);
    // let old = hm.insert(30,4); // key is going to update from value 3 to 4, returns old value
    //
    //
    // println!("{:?}",hm);
    // println!("{:?}",old);
    //
    // println!("{}", hm.contains_key(&5)); //refs if value exists in hm
    // println!("{:?}", hm.get(&5));
    //
    // let one = hm.remove(&5);
    // println!("{:?}",one);
    //
    // let remove = hm.remove_entry(&1);
    // println!("{:?}",remove);
    //
    // hm.clear();
    // println!("{:?}",hm.is_empty());

    let mut hs = HashSet::new();

    // len()
    // if_empty()

    hs.insert(1);
    hs.insert(2);
    hs.insert(3);
    hs.insert(4);

    // hs.remove(&2);

    for x in hs.iter() {
        println!("Iter: {}", x);
    }

    let mut hs2 = HashSet::new();

    hs2.insert(1);
    hs2.insert(3);
    hs2.insert(5);
    hs2.insert(7);

    for x in hs.intersection(&hs2) {
        println!("Intersection: {}", x);
    }
    // vvvvv shorthand of this ^^^^
    let intersection = &hs & &hs2; //hset with the values that are in both hs s
    for x in intersection {
        println!("Intersection: {}", x);
    }

    let union = &hs | &hs2; //hset with union of both hs
    for x in union {
        println!("Union: {}", x);
    }

    let diff = &hs - &hs2;
    let diff2 = &hs2 - &hs;
    for x in diff {
        println!("Diff: {}", x);
    }
    for x in diff2 {
        println!("Diff2: {}", x);
    }
}
