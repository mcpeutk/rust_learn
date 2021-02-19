extern crate crossbeam;

use std::ops::Add;
use std::time::Instant;

const SIZE_THRESHOLD: usize = 10; // Vec size
const TIME_THRESHOLD: u128 = 100; // Microseconds
const THREADS_NUM: usize = 3;

// Used Add trait to test functionality
fn proceed_elem<T: Add<Output = T> + Copy>(t: T) -> T {
    t + t
}

fn generic_split_on_size<T: Copy>(array: Vec<T>, proceed: fn(T) -> T) -> Vec<T> {
    let mut res_array: Vec<T> = Vec::with_capacity(array.len());

    for elem in array.iter() {
        res_array.push(*elem);
    }

    if array.len() < SIZE_THRESHOLD {
        // Don't split into threads
        for i in 0..array.len() {
            res_array[i] = proceed(array[i]);
        }

    } else {
        // Split into multiple threads
        for chunk in res_array.chunks_mut(THREADS_NUM) {
            let _ = crossbeam::scope(move |_| {
                for elem in chunk.iter_mut() {
                    *elem = proceed(*elem);
                }
            });
        }
    }  
    
    res_array
}

fn generic_split_on_time<T: Copy>(array: Vec<T>, proceed: fn(T) -> T) -> Vec<T> {
    let start = Instant::now();
    
    let mut res_array: Vec<T> = Vec::with_capacity(array.len());

    for elem in array.iter() {
        res_array.push(*elem);
    }

    for i in 0..array.len() {
        if start.elapsed().as_micros() >= TIME_THRESHOLD {
            for chunk in res_array[i..].chunks_mut(THREADS_NUM) {
                let _ = crossbeam::scope(move |_| {
                    for elem in chunk.iter_mut() {
                        *elem = proceed(*elem);
                    }
                });
            }
            break;
        }

        res_array[i] = proceed(array[i]);
    }

    res_array
}

fn main() {
    // Size threshold test
    let v = vec![1, 2, 3, 4, 5];
    println!("Initial vec: {:?}", v);

    let res_arr = generic_split_on_size(v, proceed_elem);
    println!("Result vec: {:?}\n", res_arr);

    // Elapsed time threshold test
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 
                 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 
                 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 
                 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    println!("Initial vec: {:?}", v);

    let res_arr = generic_split_on_time(v, proceed_elem);
    println!("Result vec: {:?}", res_arr);
}
