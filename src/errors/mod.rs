pub fn panic_on_condition(x: i32) {
    if x < 0 {
        panic!("x cannot be negative, got: {}", x);
    }
    println!("x is: {}", x);
}

