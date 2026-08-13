use std::borrow::Cow;
use std::mem::size_of_val;

pub fn demo() {
    println!("\n--- Cow<T> ---");

    fn check<'a>(input: &'a str) -> Cow<'a, str> {
        if input.contains(' ') {
            Cow::Owned(input.replace(' ', "_")) // allocates
        } else {
            Cow::Borrowed(input) // zero allocation
        }
    }

    let a = check("hello_world"); // Borrowed
    let b = check("hello world"); // Owned

    println!(
        "borrowed: {} | is_borrowed: {}",
        a,
        matches!(a, Cow::Borrowed(_))
    );
    println!(
        "owned:    {} | is_owned:    {}",
        b,
        matches!(b, Cow::Owned(_))
    );

    println!("Cow size: {} bytes", size_of_val(&a));
}
