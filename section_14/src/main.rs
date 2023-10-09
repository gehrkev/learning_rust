// use std::rc::Rc;
// use std::sync::Arc;
// use std::thread;
// use std::sync::mpsc;
// use std::sync::{Arc, Mutex};

use num::{BigUint, One};
use rayon::prelude::*;
use std::time::Instant;

fn factorial(num: u32) -> BigUint {
    //return this because u128 factorial goes up to only 35
    if num == 0 || num == 1 {
        return BigUint::one();
    } else {
        (1..=num)
            .map(BigUint::from)
            .reduce(|acc, x| acc * x)
            .unwrap()
    }
}

fn multi_fac(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one();
    } else {
        (1..=num)
            .into_par_iter()
            .map(BigUint::from)
            .reduce(|| BigUint::one(), |acc, x| acc * x)
        //the reduce used here is from rayon
    }
}

fn main() {
    println!("Calculating factorial of 50.000 using single thread");
    let now = Instant::now();
    factorial(50000);
    println!("Done! {:.2?}", now.elapsed());

    println!("Calculating factorial of 50.000 using multi-threading");
    let now = Instant::now();
    multi_fac(50000);
    println!("Done! {:.2?}", now.elapsed());

    // let lock = Arc::new(Mutex::new(0));
    // let lock2 = Arc::clone(&lock);
    //
    // let _ = std::thread::spawn(move || -> () {
    //     let _guard = lock2.lock().unwrap(); //we acquire the lock here
    //     panic!(); //mutex is now poisoned
    // }).join();
    //
    // let mut guard = match lock.lock() {
    //     Ok(guard) => guard,
    //     Err(poisoned) => poisoned.into_inner(),
    // };
    //
    // *guard += 1;
    // println!("{:?}", guard);

    // let handle = std::thread::spawn(move || {
    //    println!("hello from a thead!")
    // });
    //
    // handle.join().unwrap();
    //
    // println!("hello from main!");

    // let v = vec![1,2,3];

    // let handle = std::thread::spawn(move || {
    //     println!("{:?}", v);
    // });

    // let mut thread_handles = Vec::new();
    // for e in v {
    //     thread_handles.push(thread::spawn(move || println!("Thread {}", e)));
    // }
    //
    // println!("Main thread!");
    //
    // for handle in thread_handles {
    //     handle.join().unwrap();
    // }

    // let (transmiter, receiver) = mpsc::sync_channel(1000);
    // let tx = transmiter.clone();
    //
    // // let val = "Transmitting".to_string();
    // // std::thread::spawn(move || {
    // //     transmiter.send(val).unwrap();
    // // });
    // //
    // // let msg = receiver.recv().unwrap();
    // // println!("{}", msg);
    // // println!("{}", val);
    //
    // std::thread::spawn(move || {
    //     let vec = vec![String::from("Transmitting"), String::from("from"), String::from("original")];
    //     for val in vec {
    //         transmiter.send(val).unwrap();
    //     }
    // });
    //
    // std::thread::spawn(move || {
    //     let vec = vec![String::from("Clone"), String::from("is"), String::from("transmitting")];
    //     for val in vec {
    //         tx.send(val).unwrap();
    //     }
    // });
    //
    // for rec in receiver {
    //     println!("{}",rec);
    // }

    // let rc1 = Arc::new(String::from("test"));
    // let rc2 = rc1.clone();
    //
    // std::thread::spawn(move ||{
    //     rc2;
    // });

    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];
    //
    // for _ in 0..8 {
    //     let counter = Arc::clone(&counter);
    //     let handle = std::thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         // let mut num2 = counter.lock().unwrap(); //deadlock
    //         *num += 1;
    //     }); //lock is given up
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // println!("{}", counter.lock().unwrap());
}
