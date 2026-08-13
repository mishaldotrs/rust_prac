use std::sync::Arc;
use std::thread;

pub fn demo() {
    println!("\n--- Arc<T> ---");

    let data = Arc::new(String::from("shared data"));

    let d1 = Arc::clone(&data);
    let d2 = Arc::clone(&data);

    println!("count: {}", Arc::strong_count(&data)); // 3

    let t1 = thread::spawn(move || {
        println!("thread 1: {}", d1);
    });

    let t2 = thread::spawn(move || {
        println!("thread 2: {}", d2);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("main: {}", data);
    println!("count: {}", Arc::strong_count(&data)); // 1
}
