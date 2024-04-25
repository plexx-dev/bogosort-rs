#![feature(is_sorted)]

extern crate rand;
extern crate actix;

use rand::seq::SliceRandom;
use std::thread;
use std::time::Instant;

pub fn test() {
    let now = Instant::now();
    let mut rng = rand::thread_rng();

    let mut test: Vec<i32> = (0..12).collect();
    test.shuffle(&mut rng);

    let mut threads = vec![];
    for _i in 0..8 {
        let mut clone_vector = test.clone();
        let clone_now = now.clone();

        let tmp = thread::spawn(move|| { 
            clone_vector = bogo_sort(clone_vector);
            println!("{:?}", clone_vector);

            println!("Took: {} ms", clone_now.elapsed().as_millis());
            std::process::exit(1);
        });
        threads.push(tmp);
    }

    for n in threads {
        n.join().unwrap();
    }
}

fn bogo_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    loop {
        if vec.is_empty()|| vec.is_sorted() {
            return vec;
        }

        vec.shuffle(&mut rng);      
    }
}