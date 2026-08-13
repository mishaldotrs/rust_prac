use std::rc::Rc;

pub fn demo() {
    println!("\n--- Rc<T> ---");

    let a = Rc::new(String::from("hello"));
    println!("count: {}", Rc::strong_count(&a)); // 1

    let b = Rc::clone(&a);
    println!("count: {}", Rc::strong_count(&a)); // 2

    let c = Rc::clone(&a);
    println!("count: {}", Rc::strong_count(&a)); // 3

    println!("{} {} {}", a, b, c);

    drop(c);
    println!("after drop count: {}", Rc::strong_count(&a)); // 2
}
