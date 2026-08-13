use std::pin::Pin;

pub fn demo() {
    println!("\n--- Pin<T> ---");

    // pin a value on heap
    let pinned: Pin<Box<String>> = Box::pin(String::from("hello"));

    println!("value:  {}", pinned);
    println!("length: {}", pinned.len());

    // cannot move out of Pin
    // let moved = *pinned;  // ❌ ERROR

    // normal Box — can move
    let normal = Box::new(String::from("world"));
    let moved = *normal; // ✅ can move out of regular Box
    println!("moved:  {}", moved);
}
