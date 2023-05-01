use std::sync::{Arc, RwLock};

// fn sequential_fn() -> Vec<i32> {
//     let mut a = vec![1,2,3,4]; //two mutable arrays
//     let mut b = vec![0;4];
//
//     for _ in 0..100 {
//         for i in 0..4 {
//             println!("{:?} {:?} {:?}", i, a[i], b[i]);
//             b[i] ^= a[i]; //example heavy operation - a isnt modified but b is
//         }
//         b.sort();       //heavy operation over, b is sorted
//         // a is modified by main thread before repeating process
//         for e in a.iter_mut() { *e += 1; }
//     }
//     return b; //return b
// }

fn parallel_fn() -> Vec<i32> {
    let mut a = vec![1,2,3,4];
    let mut b = vec![0;4];
    let (mut b1 , mut b2) = b.split_at_mut(2); //split b into slices for each worker (err1)

    for repetitions in 0..100 {
        //only 1 worker for example purposes
        let worker = std::thread::spawn(||  //(err2)
        {
            for i in 2..4 {
                b2[i-2] ^= a[i] //mutably borrow b2, immutably borrow a
            }
        });
        for i in 0..2 {
            b1[i] ^= a[i]; // mutably borrow b1, immutably borrow a
        }
        worker.join(); //workers finish
        b.sort(); //borrow b as mutable in main thread only (err3)
        for e in a.iter_mut() //borrow a as mutable in main thread only (err4)
            { *e += 1;}
    }
    return b;
}

fn parallel_fn() -> Vec<i32> {
    let mut a = Arc::new(RwLock::new(vec![1,2,3,4]));
    let mut b = Arc::new(RwLock::new(vec![0;4]));

    for _ in 0..100 {
        //only 1 worker for example purposes
        let worker = std::thread::spawn(||  { //(err2)
            for i in 0..4 {
                b[i] ^= a[i] //mutably borrow b2, immutably borrow a
            }
        });
        worker.join(); //workers finish
        b.sort(); //borrow b as mutable in main thread only (err3)
        for e in a.iter_mut() { *e += 1; } //borrow a as mutable in main thread only (err4)
    }
    return b;
}

fn main() {
    // println!("{:?}", sequential_fn());
    println!("{:?}", parallel_fn());
}
