#![allow(dead_code)]

pub mod run_mod;

// ============================================================
// STEP 1 — TRAITS (contract kya hota hai)
// ============================================================

trait MakeSound {
    fn sound(&self) -> String; // compulsory — body nahi, sirf contract
}

struct Dog;
struct Cat;
struct Human;

impl MakeSound for Dog {
    fn sound(&self) -> String {
        String::from("Woof!")
    }
}

impl MakeSound for Cat {
    fn sound(&self) -> String {
        String::from("Meow!")
    }
}

impl MakeSound for Human {
    fn sound(&self) -> String {
        String::from("Hello!")
    }
}

// ============================================================
// STEP 2 — TRAIT IMPL (kisi struct pe implement karna)
// ============================================================

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for Dog {
    fn describe(&self) -> String {
        String::from("I am a Dog")
    }
}

impl Describe for Cat {
    fn describe(&self) -> String {
        String::from("I am a Cat")
    }
}

impl Describe for Human {
    fn describe(&self) -> String {
        String::from("I am a Human")
    }
}

// ============================================================
// STEP 3 — DEFAULT IMPL (override karna optional)
// ============================================================

trait Greet {
    fn greet(&self) -> String {
        String::from("Hi! (default greeting)") // default body
    }
}

struct Fish;

impl Greet for Human {
    fn greet(&self) -> String {
        String::from("Hello, nice to meet you!") // override
    }
}

impl Greet for Fish {} // kuch nahi — default use karega

// ============================================================
// STEP 4 — GENERICS <T>
// ============================================================

// Generic function
fn print_it<T: std::fmt::Display>(a: T) {
    println!("{}", a);
}

// Generic struct
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T: std::fmt::Display, U: std::fmt::Display> Pair<T, U> {
    fn show(&self) {
        println!("first: {}, second: {}", self.first, self.second);
    }
}

// Generic function with two different types
fn sum_generic<T: std::ops::Add<U>, U>(x: T, y: U) -> T::Output {
    x + y
}

// ============================================================
// STEP 5 — TRAIT BOUNDS (<T: SomeTrait>)
// ============================================================

use std::fmt::{Debug, Display};

// Single bound
fn print_display<T: Display>(a: T) {
    println!("Display: {}", a);
}

// Multiple bounds
fn print_both<T: Display + Debug>(a: T) {
    println!("Display: {}  |  Debug: {:?}", a, a);
}

// where clause — same cheez, clean syntax
fn print_where<T>(a: T)
where
    T: Display + Debug,
{
    println!("where clause → Display: {}  |  Debug: {:?}", a, a);
}

// Apna trait as bound
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

struct Tweet {
    username: String,
    message: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.message)
    }
}

fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// ============================================================
// STEP 6 — impl Trait (shorthand for trait bounds)
// ============================================================

// parameter mein
fn notify_impl(item: impl Summary) {
    println!("{}", item.summarize());
}

// return type mein
fn get_article() -> impl Summary {
    Article {
        title: String::from("impl Trait is cool"),
        content: String::from("shorthand for trait bounds"),
    }
}

// ============================================================
// STEP 7 — dyn Trait (runtime polymorphism)
// ============================================================

// runtime pe type decide — Box<dyn Trait>
fn get_animal(name: &str) -> Box<dyn MakeSound> {
    match name {
        "dog" => Box::new(Dog),
        "cat" => Box::new(Cat),
        _ => Box::new(Human),
    }
}

// &dyn Trait — reference se
fn print_sound(animal: &dyn MakeSound) {
    println!("{}", animal.sound());
}

// ============================================================
// STEP 8 — Associated Types
// ============================================================

trait Converter {
    type Output; // associated type
    fn convert(&self) -> Self::Output;
}

struct Celsius(f64);
struct Km(f64);

impl Converter for Celsius {
    type Output = f64;
    fn convert(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

impl Converter for Km {
    type Output = f64;
    fn convert(&self) -> f64 {
        self.0 * 0.621371
    }
}

// ============================================================
// STEP 9 — Blanket impl
// ============================================================

trait PrintMe {
    fn print_me(&self);
}

// jo bhi Display implement kare — uske liye PrintMe automatically
impl<T: std::fmt::Display> PrintMe for T {
    fn print_me(&self) {
        println!("Value: {}", self);
    }
}

// ============================================================
// (run() moved to run_mod.rs)
// ============================================================

fn _unused() {
    println!("\n===== STEP 1 & 2 — TRAITS & IMPL =====");
    let d = Dog;
    let c = Cat;
    let h = Human;
    println!("{}", d.sound());
    println!("{}", c.sound());
    println!("{}", h.sound());
    println!("{}", d.describe());
    println!("{}", c.describe());
    println!("{}", h.describe());

    println!("\n===== STEP 3 — DEFAULT IMPL =====");
    let f = Fish;
    println!("{}", h.greet()); // override
    println!("{}", f.greet()); // default

    println!("\n===== STEP 4 — GENERICS =====");
    print_it(42);
    print_it(3.14);
    print_it("hello");
    let p = Pair {
        first: 10,
        second: "world",
    };
    p.show();
    println!("sum_generic: {}", sum_generic(10, 20));
    println!("sum_generic: {}", sum_generic(1.5, 2.5));

    println!("\n===== STEP 5 — TRAIT BOUNDS =====");
    print_display(100);
    print_both("rust");
    print_where(42);
    let a = Article {
        title: String::from("Rust is amazing"),
        content: String::from("Rust is fast and safe"),
    };
    let t = Tweet {
        username: String::from("rustlang"),
        message: String::from("Exciting new features!"),
    };
    notify(a);
    notify(t);

    println!("\n===== STEP 6 — impl Trait =====");
    let a2 = Article {
        title: String::from("Rust is amazing"),
        content: String::from("fast and safe"),
    };
    notify_impl(a2);
    let s = get_article();
    println!("{}", s.summarize());

    println!("\n===== STEP 7 — dyn Trait =====");
    // Box<dyn Trait>
    let a1 = get_animal("dog");
    let a2 = get_animal("cat");
    let a3 = get_animal("xyz");
    println!("{}", a1.sound());
    println!("{}", a2.sound());
    println!("{}", a3.sound());

    // Vec<Box<dyn Trait>> — alag alag types ek vector mein
    let animals: Vec<Box<dyn MakeSound>> = vec![Box::new(Dog), Box::new(Cat), Box::new(Human)];
    for animal in &animals {
        print_sound(animal.as_ref());
    }

    println!("\n===== STEP 8 — Associated Types =====");
    let c = Celsius(100.0);
    let k = Km(10.0);
    println!("100°C = {}°F", c.convert());
    println!("10 km = {} miles", k.convert());

    println!("\n===== STEP 9 — Blanket impl =====");
    42.print_me();
    3.14.print_me();
    "hello".print_me();
    true.print_me();
}
