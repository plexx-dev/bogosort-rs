extern crate rand;

use rand::seq::SliceRandom;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::thread;
use std::time::{Duration, Instant};

fn bogo_sort(mut vec: Vec<i32>, terminated: Arc<AtomicBool>) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    loop {
        if terminated.load(Ordering::Relaxed) {
            return vec;
        }

        if vec.is_sorted() {
            return vec;
        }

        vec.shuffle(&mut rng);
    }
}

pub fn test_avg(n: i32, len: i32) {
    let mut time_sum: Duration = Duration::new(0, 0);

    for i in 0..n {
        let time = test(len);
        time_sum += time;
        println!("{}: {}ms", i, time.as_millis());
    }

    println!(
        "arr len: {} runs: {}, avg: {}ms",
        len,
        n,
        time_sum.as_millis() / (n as u128)
    );
}

pub fn test(len: i32) -> Duration {
    let mut rng = rand::thread_rng();
    let mut test: Vec<i32> = (0..len).collect();
    test.shuffle(&mut rng);

    let (tx, rx) = mpsc::channel::<(Vec<i32>, Duration)>();
    let test_shared = Arc::new(test);
    let terminated = Arc::new(AtomicBool::new(false));

    let mut handles = vec![];
    for _i in 0..8 {
        let test_clone = Arc::clone(&test_shared);
        let tx_clone = mpsc::Sender::clone(&tx);
        let terminated_clone = Arc::clone(&terminated);

        let handle = thread::spawn(move || {
            let thread_now = Instant::now();
            let sorted_vec = bogo_sort(test_clone.to_vec(), terminated_clone);
            let thread_elapsed = thread_now.elapsed();

            if sorted_vec.is_sorted() {
                let _ = tx_clone.send((sorted_vec, thread_elapsed));
            }
        });
        handles.push(handle);
    }

    let (_, first_elapsed_time) = rx.recv().unwrap();

    terminated.store(true, Ordering::Relaxed);

    for handle in handles {
        handle.join().unwrap();
    }

    first_elapsed_time
}
