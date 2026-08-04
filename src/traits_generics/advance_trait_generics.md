# Advanced Traits & Generics
# Base example used everywhere below:
#
# trait Greet {
#     fn say_hello(&self) { println!("hello"); }
# }
# struct GoodMorning;
# impl Greet for GoodMorning {
#     fn say_hello(&self) { println!("good morning ji"); }
# }

---------------------------------------------------------------------------------------------------------

## 1. Supertraits

A trait that REQUIRES another trait to be implemented first.
"Before you implement Greet, you must also implement Display."

```rust
use std::fmt::Display;

trait Greet: Display {          // Greet requires Display — supertrait
    fn say_hello(&self) {
        println!("Hello, I am {}", self);   // self se Display use kar sakte hain
    }
}

struct GoodMorning;

impl Display for GoodMorning {              // pehle Display implement karo
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GoodMorning")
    }
}

impl Greet for GoodMorning {}               // ab Greet implement kar sakte ho

fn main() {
    let g = GoodMorning;
    g.say_hello();   // Hello, I am GoodMorning
}
```

Rule: You cannot implement Greet without implementing Display first.

---------------------------------------------------------------------------------------------------------

## 2. Marker Traits

Traits with NO methods — they just "tag" a type to say it has some property.

```rust
trait CanGreet {}       // no methods — just a marker

struct GoodMorning;
struct GoodNight;

impl CanGreet for GoodMorning {}   // GoodMorning is tagged as CanGreet
// GoodNight is NOT tagged

fn only_greeters<T: CanGreet>(item: T) {
    println!("this type is allowed to greet");
}

fn main() {
    only_greeters(GoodMorning);   // OK
    // only_greeters(GoodNight);  // ERROR — GoodNight is not CanGreet
}
```

Real examples from std library:
- Copy    → value is copied instead of moved
- Send    → safe to send across threads
- Sync    → safe to share reference across threads
- Sized   → size known at compile time

---------------------------------------------------------------------------------------------------------

## 3. Sized & ?Sized

Every generic T is implicitly Sized — Rust assumes its size is known at compile time.

```rust
fn foo<T>(x: T) {}
// same as:
fn foo<T: Sized>(x: T) {}   // Rust adds this automatically
```

?Sized means "maybe not Sized" — opt out of the assumption.

```rust
trait Greet {
    fn say_hello(&self);
}

// T: ?Sized — T can be a dynamically sized type like str, [u8], dyn Trait
fn greet_any<T: ?Sized + Greet>(item: &T) {
    item.say_hello();
}
```

When you need ?Sized:
- Working with str (not String)
- Working with [T] (not Vec<T>)
- Working with dyn Trait

---------------------------------------------------------------------------------------------------------

## 4. Orphan Rule

You can only implement a trait for a type if:
- The TRAIT is defined in your crate, OR
- The TYPE is defined in your crate.
You cannot implement an external trait for an external type.

```rust
// ALLOWED — Greet is YOUR trait, GoodMorning is YOUR type
impl Greet for GoodMorning { ... }

// ALLOWED — Greet is YOUR trait, i32 is external
impl Greet for i32 { ... }

// ALLOWED — Display is external, GoodMorning is YOUR type
impl std::fmt::Display for GoodMorning { ... }

// NOT ALLOWED — both Display and i32 are external (std library)
impl std::fmt::Display for i32 { ... }   // ERROR: orphan rule
```

Why this rule exists:
- Prevents two different crates from implementing the same trait for the same type
- Avoids conflicts and ambiguity

---------------------------------------------------------------------------------------------------------

## 5. Object Safety

Not every trait can be used as dyn Trait.
A trait is object-safe only if:
- It has no generic methods
- It does not return Self
- All methods take &self or &mut self (not self by value with sized requirement)

```rust
// OBJECT SAFE — can use as dyn Greet
trait Greet {
    fn say_hello(&self);
}

// NOT OBJECT SAFE — returns Self
trait Clone {
    fn clone(&self) -> Self;   // Self size unknown at runtime — CANNOT use dyn Clone
}

// NOT OBJECT SAFE — generic method
trait Printer {
    fn print<T>(&self, item: T);   // generic — CANNOT use dyn Printer
}
```

```rust
let g: Box<dyn Greet> = Box::new(GoodMorning);   // OK — Greet is object safe
```

---------------------------------------------------------------------------------------------------------

## 6. Turbofish

Explicitly tell Rust what the generic type is using ::<> syntax.

```rust
fn greet_typed<T: std::fmt::Debug>(val: T) {
    println!("{:?}", val);
}

fn main() {
    greet_typed(42);              // Rust infers T = i32
    greet_typed::<i32>(42);       // turbofish — you explicitly say T = i32
    greet_typed::<f64>(3.14);     // T = f64

    // common use with collect()
    let nums = vec!["1", "2", "3"];
    let parsed = nums.iter()
        .map(|s| s.parse::<i32>().unwrap())   // turbofish here
        .collect::<Vec<i32>>();               // and here
}
```

When you need turbofish:
- Rust cannot infer the type on its own
- You want to be explicit and clear
- Using .parse(), .collect(), or similar methods

---------------------------------------------------------------------------------------------------------

## 7. impl blocks on Generics

Add methods to a generic struct — only for specific types or all types.

```rust
struct Greeter<T> {
    value: T,
}

// for ALL types T
impl<T> Greeter<T> {
    fn new(value: T) -> Self {
        Greeter { value }
    }
}

// only for T that implements Display
impl<T: std::fmt::Display> Greeter<T> {
    fn greet(&self) {
        println!("Hello from {}", self.value);
    }
}

fn main() {
    let g = Greeter::new("GoodMorning");
    g.greet();   // Hello from GoodMorning

    let n = Greeter::new(42);
    n.greet();   // Hello from 42
}
```

---------------------------------------------------------------------------------------------------------

## 8. Const Generics

Generic over a VALUE (not a type) — known at compile time.

```rust
struct Greetings<const N: usize> {
    messages: [&'static str; N],   // array size is generic!
}

impl<const N: usize> Greetings<N> {
    fn print_all(&self) {
        for msg in &self.messages {
            println!("{}", msg);
        }
    }
}

fn main() {
    let g = Greetings {
        messages: ["hello", "good morning", "hi"],
    };
    g.print_all();
}
```

Real use — std library arrays use const generics:
```rust
impl<T, const N: usize> [T; N] { ... }
// that's why [i32; 3] and [i32; 5] are different types
```

---------------------------------------------------------------------------------------------------------

## 9. GATs — Generic Associated Types

Associated types that themselves have generic parameters (lifetime or type).
Stabilized in Rust 1.65.

```rust
trait Greetable {
    type Output<'a>                // associated type with a lifetime generic
    where Self: 'a;

    fn get_greeting<'a>(&'a self) -> Self::Output<'a>;
}

struct GoodMorning {
    message: String,
}

impl Greetable for GoodMorning {
    type Output<'a> = &'a str;     // returns a reference tied to self's lifetime

    fn get_greeting<'a>(&'a self) -> &'a str {
        &self.message
    }
}

fn main() {
    let g = GoodMorning { message: String::from("good morning ji") };
    println!("{}", g.get_greeting());
}
```

When you need GATs:
- Returning references from trait methods
- Building generic iterators that borrow from self

---------------------------------------------------------------------------------------------------------

## 10. HRTB — Higher-Ranked Trait Bounds

"This trait must work for ANY lifetime, not just a specific one."
Written as: for<'a> T: Trait<'a>

```rust
// normal bound — works for ONE specific lifetime
fn call<'a, F: Fn(&'a str)>(f: F, s: &'a str) {
    f(s);
}

// HRTB — works for ANY lifetime
fn call_any<F: for<'a> Fn(&'a str)>(f: F) {
    f("hello");
    f("good morning");
}

fn main() {
    call_any(|s| println!("{}", s));
}
```

When you see it:
- Mostly in closures and function pointers
- Rust often infers it automatically — you rarely write it manually
- Appears in trait objects: Box<dyn for<'a> Fn(&'a str)>

---------------------------------------------------------------------------------------------------------

## 11. PhantomData

A zero-sized marker that tells Rust "this struct logically owns/uses type T"
even though T does not appear in any real field.

```rust
use std::marker::PhantomData;

struct Greeter<T> {
    message: String,
    _marker: PhantomData<T>,   // T is not used in any real field
}

impl<T> Greeter<T> {
    fn new(msg: &str) -> Self {
        Greeter {
            message: msg.to_string(),
            _marker: PhantomData,
        }
    }

    fn say(&self) {
        println!("{}", self.message);
    }
}

fn main() {
    let g: Greeter<GoodMorning> = Greeter::new("good morning");
    g.say();
}
```

Why it exists:
- Tell the compiler about ownership/variance of T
- Prevent "unused generic parameter" errors
- Used heavily in unsafe code and low-level libraries

---------------------------------------------------------------------------------------------------------

## 12. Sealed Traits

A PATTERN (not a keyword) — prevents anyone outside your module from implementing your trait.

```rust
mod private {
    pub trait Sealed {}           // private supertrait
}

pub trait Greet: private::Sealed {    // Greet requires Sealed
    fn say_hello(&self);
}

pub struct GoodMorning;

impl private::Sealed for GoodMorning {}   // only YOU can implement Sealed
impl Greet for GoodMorning {
    fn say_hello(&self) { println!("good morning"); }
}

// Outside users:
// impl Greet for TheirType { ... }   // ERROR — cannot implement Sealed
```

When to use:
- You want to add methods to a trait later without breaking users
- You want full control over which types implement your trait

---------------------------------------------------------------------------------------------------------

## 13. Auto Traits

Traits that Rust implements AUTOMATICALLY for a type if all its fields satisfy the trait.
You don't write impl — Rust does it for you.

```rust
struct GoodMorning {
    message: String,   // String is Send + Sync
}

// Rust automatically implements:
// impl Send for GoodMorning {}
// impl Sync for GoodMorning {}
// because all fields (String) are Send + Sync
```

The main auto traits:
- Send  → safe to transfer ownership to another thread
- Sync  → safe to share &T across threads
- Sized → size known at compile time (almost everything)

Opting out:
```rust
impl !Send for MyType {}   // manually remove Send (rare, unsafe)
```

---------------------------------------------------------------------------------------------------------

## 14. Specialization (unstable — nightly only)

Allows a more specific impl to override a general blanket impl.
Still experimental — not in stable Rust.

```rust
// general impl for all T
impl<T: std::fmt::Display> Greet for T {
    fn say_hello(&self) {
        println!("Hello {}", self);
    }
}

// specialized impl just for GoodMorning (overrides the general one)
impl Greet for GoodMorning {
    fn say_hello(&self) {
        println!("Very special good morning!");
    }
}
```

Status: Only on nightly Rust with #![feature(specialization)]
Use case: Library authors who need fine-grained control over implementations.

---------------------------------------------------------------------------------------------------------

## Summary

Intermediate:
  Supertraits        → trait requires another trait first
  Marker Traits      → no methods, just a tag (Copy, Send, Sync)
  Sized / ?Sized     → Sized is implicit, ?Sized opts out
  Orphan Rule        → your trait OR your type, not both external
  Object Safety      → rules for using dyn Trait
  Turbofish          → foo::<Type>() explicit type hint
  impl on Generics   → add methods to generic structs

Advanced:
  Const Generics     → generic over a VALUE not a type
  GATs               → associated types with their own generics
  HRTB               → for<'a> — works for any lifetime
  PhantomData        → zero-sized marker for unused generics
  Sealed Traits      → pattern to block external implementations
  Auto Traits        → Rust implements automatically (Send, Sync)
  Specialization     → unstable — specific impl overrides general
