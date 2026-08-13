pub fn demo() {
    println!("\n--- Box<T> ---");

    // heap allocation
    let b = Box::new(42);
    println!("value: {}", b);
    println!("deref: {}", *b);

    // Box with String
    let s = Box::new(String::from("hello"));
    println!("string: {}", s);
    println!("length: {}", s.len());

    // Box<dyn Error>
    fn might_fail(fail: bool) -> Result<String, Box<dyn std::error::Error>> {
        if fail {
            Err("something went wrong".into())
        } else {
            Ok(String::from("success"))
        }
    }

    println!("{:?}", might_fail(false));
    println!("{:?}", might_fail(true));
}
