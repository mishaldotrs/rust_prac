use std::cell::RefCell;

pub fn demo() {
    println!("\n--- RefCell<T> ---");

    let data = RefCell::new(5);

    // immutable borrow
    let r1 = data.borrow();
    let r2 = data.borrow();
    println!("r1: {} r2: {}", r1, r2);
    drop(r1);
    drop(r2);

    // mutable borrow
    *data.borrow_mut() += 10;
    println!("after mutation: {}", data.borrow());

    // interior mutability in struct
    struct Counter {
        count: RefCell<u32>,
    }

    impl Counter {
        fn increment(&self) {
            *self.count.borrow_mut() += 1;
        }
        fn value(&self) -> u32 {
            *self.count.borrow()
        }
    }

    let c = Counter {
        count: RefCell::new(0),
    };
    c.increment();
    c.increment();
    c.increment();
    println!("counter: {}", c.value()); // 3
}
