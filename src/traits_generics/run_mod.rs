use super::*;

pub fn run() {
    println!("\n===== 1 & 2. TRAITS & IMPL =====");
    let d = Dog;
    let c = Cat;
    let h = Human;
    println!("{}", d.sound());
    println!("{}", c.sound());
    println!("{}", h.sound());
    println!("{}", d.describe());
    println!("{}", c.describe());
    println!("{}", h.describe());

    println!("\n===== 3. DEFAULT IMPL =====");
    let f = Fish;
    println!("{}", h.greet());
    println!("{}", f.greet());

    println!("\n===== 4. GENERICS =====");
    print_it(42);
    print_it(3.14);
    print_it("hello");
    let p = Pair {
        first: 10,
        second: "world",
    };
    p.show();
    println!("sum_generic: {}", sum_generic(10, 20));

    println!("\n===== 5. TRAIT BOUNDS =====");
    print_display(100);
    print_both("rust");
    print_where(42);
    let a = Article {
        title: String::from("Rust is amazing"),
        content: String::from("fast and safe"),
    };
    let t = Tweet {
        username: String::from("rustlang"),
        message: String::from("Exciting new features!"),
    };
    notify(a);
    notify(t);

    println!("\n===== 6. impl Trait =====");
    let a2 = Article {
        title: String::from("impl Trait"),
        content: String::from("shorthand for bounds"),
    };
    notify_impl(a2);
    let s = get_article();
    println!("{}", s.summarize());

    println!("\n===== 7. dyn Trait =====");
    let a1 = get_animal("dog");
    let a2 = get_animal("cat");
    let a3 = get_animal("xyz");
    println!("{}", a1.sound());
    println!("{}", a2.sound());
    println!("{}", a3.sound());
    let animals: Vec<Box<dyn MakeSound>> = vec![Box::new(Dog), Box::new(Cat), Box::new(Human)];
    for animal in &animals {
        print_sound(animal.as_ref());
    }

    println!("\n===== 8. Associated Types =====");
    let c = Celsius(100.0);
    let k = Km(10.0);
    println!("100°C = {}°F", c.convert());
    println!("10 km = {} miles", k.convert());

    println!("\n===== 9. Blanket impl =====");
    42.print_me();
    3.14.print_me();
    "hello".print_me();
    true.print_me();
}
