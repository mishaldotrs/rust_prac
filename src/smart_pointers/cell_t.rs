use std::cell::Cell;

pub fn demo() {
    println!("\n--- Cell<T> ---");

    let data = Cell::new(5);
    println!("initial: {}", data.get());

    data.set(10);
    println!("after set: {}", data.get());

    // interior mutability in struct
    struct Config {
        debug: Cell<bool>,
        port: u16,
    }

    impl Config {
        fn enable_debug(&self) {
            self.debug.set(true);
        }
        fn is_debug(&self) -> bool {
            self.debug.get()
        }
    }

    let config = Config {
        debug: Cell::new(false),
        port: 8080,
    };
    println!("debug before: {}", config.is_debug());
    config.enable_debug();
    println!("debug after:  {}", config.is_debug());
    println!("port: {}", config.port);
}
