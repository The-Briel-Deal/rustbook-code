use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result = {}", *counter.lock().unwrap());
}

/*
 * let m = Mutex::new(5);
 * let mut num = m.lock().unwrap();
 * *num = 8;
 * drop(num);
 * println!("m = {m:?}")
 */
