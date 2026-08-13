use std::sync::{Arc, Mutex};
use std::thread;

pub fn demo() {
    println!("\n--- Arc<Mutex<T>> ---");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("thread {}: counter = {}", i, num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("final: {}", counter.lock().unwrap());
}
