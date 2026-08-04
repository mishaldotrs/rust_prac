trait Greetings {
    fn greetings(&self);
}

struct English;
struct Hindi;

impl Greetings for English {
    fn greetings(&self) {
        println!("good morning");
    }
}

impl Greetings for Hindi {
    fn greetings(&self) {
        println!("namaste");
    }
}

// Accepts any type that implements Greetings
fn say_hello(g: &impl Greetings) {
    g.greetings();
}

// Alternative generic form (same idea)
fn say_hello_generic<T: Greetings>(g: &T) {
    g.greetings();
}

fn main() {
    let english_greet = English;
    let hindi_greet = Hindi;

    // Call the method directly
    english_greet.greetings();
    hindi_greet.greetings();

    // Or use the helper function (polymorphism)
    say_hello(&english_greet);
    say_hello(&hindi_greet);

    // Same with the generic version
    say_hello_generic(&english_greet);
    say_hello_generic(&hindi_greet);
}