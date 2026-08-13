use std::sync::Mutex;

pub fn demo() {
    println!("\n--- Mutex<T> ---");

    let data = Mutex::new(5);

    {
        let mut guard = data.lock().unwrap();
        *guard += 10;
        println!("inside lock: {}", guard);
    } // lock released here

    println!("after lock: {}", data.lock().unwrap());
}
