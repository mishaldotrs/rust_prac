# Traits & Generics — Complete Theory Guide

> Code reference: [traits_generic/mod.rs](https://github.com/mishaldotrs/rust_prac/blob/main/src/traits_generics/mod.rs)

---

## 0. Definitions

### Generics
**Generics** allow you to write code (functions, structs, enums, methods) that
works with **any type**, instead of writing separate code for each concrete
type. The concrete type is decided at **compile time**, based on how the
generic is used.

```rust
fn identity<T>(x: T) -> T {
    x
}
```

### Traits
A **trait** defines a set of methods that a type must implement — it's a
**contract**. Any type that implements the trait guarantees it provides that
behavior. This is Rust's mechanism for **shared behavior / polymorphism**,
similar to interfaces in Java/C# or protocols in Swift.

```rust
trait Speak {
    fn speak(&self) -> String;
}
```

**Generics answer:** "What type is this?"
**Traits answer:** "What can this type do?"

They are almost always used **together**: generics describe "any type", and
trait bounds restrict that "any type" to "any type that can do X".

---

## 1. Why do we need them? (The actual problem)

Without generics/traits, you'd have to duplicate code for every type:

```rust
fn add_i32(a: i32, b: i32) -> i32 { a + b }
fn add_f64(a: f64, b: f64) -> f64 { a + b }
fn add_u8(a: u8, b: u8) -> u8 { a + b }
// ... forever
```

And without traits, you have no way to say "this function works on any type
that supports addition/printing/comparison/etc." — you'd need to hardcode
behavior per type, with no abstraction, no polymorphism, no code reuse.

**Generics solve:** code duplication across types.
**Traits solve:** describing shared capabilities across unrelated types, and
enabling both compile-time (static) and runtime (dynamic) polymorphism.

Together, they let Rust achieve **zero-cost abstraction** — you write generic,
reusable code, but the compiler generates specialized, fast machine code for
each concrete type used (this process is called **monomorphization**).

---

## 2. Topics 1 → 9

### 1. Traits (the contract)

A trait declares method signatures without (necessarily) providing bodies.

```rust
trait MakeSound {
    fn sound(&self) -> String;
}
```

This says: "any type implementing `MakeSound` must provide a `sound` method
returning a `String`." No implementation, no data — just a promise.

---

### 2. Trait Implementation (`impl Trait for Type`)

You fulfill the contract per type using `impl Trait for Type`.

```rust
struct Dog;

impl MakeSound for Dog {
    fn sound(&self) -> String {
        String::from("Woof!")
    }
}
```

Each type can implement the trait differently — this is **polymorphism**:
same method name (`sound`), different behavior per type.

---

### 3. Default Implementations

A trait method can have a default body. Implementers can use it as-is or
override it.

```rust
trait Greet {
    fn greet(&self) -> String {
        String::from("Hi!")   // default
    }
}

impl Greet for Fish {}          // uses default
impl Greet for Human {
    fn greet(&self) -> String { // overrides
        String::from("Hello there!")
    }
}
```

**Why:** reduces boilerplate when most types share common behavior, while
still allowing exceptions.

---

### 4. Generics (`<T>`)

A placeholder type parameter, resolved at compile time based on usage.

```rust
fn print_it<T>(a: T) { ... }
struct Pair<T, U> { first: T, second: U }
```

Key idea: **one definition, many concrete types**, with no runtime cost
(monomorphization generates a separate compiled version per type used).

---

### 5. Trait Bounds (`<T: SomeTrait>`)

Generics alone give *no guarantees* about what `T` can do. Trait bounds
restrict `T` to types implementing specific trait(s).

```rust
fn print_it<T: std::fmt::Display>(a: T) {
    println!("{}", a);   // only compiles because Display is guaranteed
}

// multiple bounds
fn foo<T: Display + Debug>(a: T) { ... }

// where clause (same meaning, better readability for complex bounds)
fn foo<T>(a: T) where T: Display + Debug { ... }
```

This is the point where **generics and traits truly combine**.

---

### 6. `impl Trait` (syntactic sugar)

A shorthand for simple trait-bound generics, usable in argument and return
position.

```rust
// argument position — same as <T: Summary>(item: T)
fn notify(item: impl Summary) { ... }

// return position — "returns some type that implements Summary"
fn get_summary() -> impl Summary { Article { ... } }
```

**Limitation:** in return position, only **one concrete type** can actually be
returned — the compiler must know the exact type at compile time (still
static dispatch under the hood).

---

### 7. `dyn Trait` (trait objects / dynamic dispatch)

Where `impl Trait` fails (returning *different* concrete types conditionally,
or storing heterogeneous types together), `dyn Trait` allows the concrete
type to be resolved **at runtime** via a vtable (virtual method table).

```rust
fn get_animal(name: &str) -> Box<dyn Sound> {
    match name {
        "dog" => Box::new(Dog),
        _     => Box::new(Cat),
    }
}

let animals: Vec<Box<dyn Sound>> = vec![Box::new(Dog), Box::new(Cat)];
```

`dyn Trait` has an unknown size at compile time (different implementers have
different sizes), so it must be used behind a pointer: `Box<dyn Trait>`,
`&dyn Trait`, `Rc<dyn Trait>`, etc.

**Static dispatch (generics/impl Trait)** vs **dynamic dispatch (dyn Trait)**:

| | Static (generics) | Dynamic (`dyn`) |
|---|---|---|
| Resolved | compile time | runtime |
| Speed | faster (inlined) | slightly slower (vtable lookup) |
| Binary size | larger (one copy per type) | smaller (one shared impl) |
| Heterogeneous collections | ❌ not possible | ✅ possible |
| Flexibility | less | more |

---

### 8. Associated Types

A trait can declare a placeholder type that's fixed once per implementation
(not per call site, unlike generic trait parameters).

```rust
trait Converter {
    type Output;
    fn convert(&self) -> Self::Output;
}

impl Converter for Celsius {
    type Output = f64;
    fn convert(&self) -> f64 { ... }
}
```

**Why not use a generic trait `Converter<Output>` instead?**
- With a generic trait, a single type could implement `Converter<f64>` *and*
  `Converter<String>` simultaneously — ambiguous, and callers must specify
  which one they want.
- With an associated type, each type gets **exactly one** implementation,
  and the output type is inferred automatically — much simpler call sites.

Rule of thumb: **if there should be exactly one implementation per type, use
an associated type. If a type may need multiple implementations parameterized
differently, use a generic trait.**

---

### 9. Blanket Implementations

Implementing a trait for **every type** that satisfies some bound, in one
single `impl` block — instead of implementing it individually per type.

```rust
impl<T: std::fmt::Display> PrintMe for T {
    fn print_me(&self) {
        println!("Value: {}", self);
    }
}
```

Now *any* current or future type that implements `Display` automatically
gets `PrintMe` — no need to write `impl PrintMe for X` for every `X`.

This is how Rust's standard library implements things like:
```rust
impl<T: Display> ToString for T { ... }
```
which is why `.to_string()` "just works" on `i32`, `f64`, `&str`, and any
custom type you make `Display`.

---

## 3. Where these show up in real Rust code

| Topic | Real-world usage |
|---|---|
| **Associated Types** | `Iterator` trait: `type Item; fn next(&mut self) -> Option<Self::Item>;` — every iterator declares what it yields. |
| **Associated Types** | `std::ops::Add`: `type Output;` — lets `+` return a different type than the operands (e.g. `Duration + Instant = Instant`). |
| **Associated Types** | `Deref`: `type Target;` — powers `*my_box` and auto-deref for smart pointers like `Box<T>`, `Rc<T>`. |
| **Associated Types** | `sqlx::FromRow` / `serde::Deserialize` — associate a target output type with a decode/deserialize operation. |
| **Trait Bounds** | Async Rust: `fn spawn<F: Future + Send + 'static>(fut: F)` — Tokio requires futures to be `Send` to move across threads. |
| **Trait Bounds** | Generic collections: `fn largest<T: PartialOrd>(list: &[T]) -> &T` |
| **dyn Trait** | Error handling: `Box<dyn std::error::Error>` — a function can return *any* error type without knowing it upfront. |
| **dyn Trait** | Callback storage: `Vec<Box<dyn Fn(i32) -> i32>>` — plugin systems, event handlers. |
| **dyn Trait** | GUI/game frameworks: `Vec<Box<dyn Widget>>`, `Vec<Box<dyn Component>>` — heterogeneous object collections. |
| **Blanket impl** | `impl<T: Display> ToString for T` (already covers most primitives). |
| **Blanket impl** | `impl<T: Iterator> IntoIterator for T` — every `Iterator` is automatically an `IntoIterator`. |
| **Generics** | `Vec<T>`, `HashMap<K, V>`, `Option<T>`, `Result<T, E>` — the backbone of the entire standard library. |
| **impl Trait** | `fn numbers() -> impl Iterator<Item = i32>` — returning complex iterator chains without spelling out their real type. |

Understanding these 9 concepts is essentially a prerequisite for reading
**any** non-trivial Rust codebase, including `tokio`, `serde`, `sqlx`,
`bevy`, and the standard library itself.

---

## 4. Interview-Focused Questions

1. What is the difference between a trait and an interface (in languages like
   Java)? What can Rust traits do that interfaces typically can't (e.g.
   default methods, operator overloading via traits)?
2. What is monomorphization? How does it relate to "zero-cost abstractions"?
3. Explain the difference between static dispatch and dynamic dispatch in
   Rust. When would you prefer one over the other?
4. Why can't you have a variable of type `dyn Trait` directly (without `&` or
   `Box`)? (Hint: unknown size at compile time — `Sized` trait.)
5. What is a trait object's vtable, and what does it store?
6. What's the difference between `impl Trait` as a parameter vs. `impl Trait`
   as a return type?
7. Why can't a function return different concrete types using `-> impl
   Trait`, but can using `-> Box<dyn Trait>`?
8. What is an associated type, and how is it different from a generic type
   parameter on a trait? Give an example of when you'd need each.
9. Explain the orphan rule: why can't you `impl` a foreign trait for a
   foreign type (e.g. `impl Display for Vec<T>` in your own crate)?
10. What is a blanket implementation? Why must you be careful about
    conflicting blanket impls (coherence rules)?
11. Can a trait have generic methods, and does that affect whether the trait
    is object-safe (i.e. usable as `dyn Trait`)? (Hint: no — generic methods
    make a trait non-object-safe.)
12. What is trait object safety? Name a couple of rules a trait must follow
    to be used as `dyn Trait`.
13. What's the difference between `<T: Trait>` and `where T: Trait`? When
    would you prefer `where`?
14. How does the `?Sized` bound relate to generics (e.g. `fn foo<T: ?Sized>`)?
15. Explain how `Iterator`'s associated type `Item` allows `for x in
    my_iter` to work without knowing the element type upfront.

---

## 5. Hands-on Exercises

Do these in order — each builds on the previous concept.

### Exercise 1 — Traits + Impl
Create a trait `Shape` with a method `area(&self) -> f64`. Implement it for
`Circle { radius: f64 }` and `Rectangle { width: f64, height: f64 }`.

### Exercise 2 — Default Implementation
Add a default method `describe(&self) -> String` to `Shape` that returns
`"This is a shape"`. Override it only for `Circle` to return `"This is a
circle"`.

### Exercise 3 — Generics
Write a generic function `largest<T: PartialOrd + Copy>(list: &[T]) -> T`
that returns the largest element in a slice of any comparable type. Test it
with `Vec<i32>` and `Vec<f64>`.

### Exercise 4 — Trait Bounds
Write a function `print_shape_info<T: Shape>(shape: &T)` that prints the
area and description of any shape using your `Shape` trait from Exercise 1.

### Exercise 5 — `impl Trait`
Rewrite `print_shape_info` using `impl Trait` syntax instead of `<T: Shape>`.
Then write a function `make_circle(radius: f64) -> impl Shape` that returns a
`Circle`.

### Exercise 6 — `dyn Trait`
Create a `Vec<Box<dyn Shape>>` containing a mix of `Circle`s and
`Rectangle`s. Loop through it and print the total area of all shapes
combined.

### Exercise 7 — Associated Types
Create a trait `Container` with `type Item; fn get(&self, index: usize) ->
Option<&Self::Item>;`. Implement it for a custom struct wrapping a `Vec<T>`.

### Exercise 8 — Blanket impl
Create a trait `Loud` with a method `shout(&self) -> String`. Write a
blanket implementation: any type implementing `Display` should get `shout()`
for free, returning its display value in UPPERCASE with 3 exclamation marks
(e.g. `"HELLO!!!"`).

### Exercise 9 — Capstone (combine everything)
Build a small "plugin system":
- Trait `Plugin` with `fn name(&self) -> &str` and `fn execute(&self) ->
  String`.
- At least 3 different structs implementing `Plugin`.
- A `PluginManager` struct holding `Vec<Box<dyn Plugin>>`.
- A method `run_all(&self)` that calls `execute()` on every plugin and
  prints `"{name}: {result}"`.
- Bonus: add a generic function `register<T: Plugin + 'static>(&mut self,
  plugin: T)` on `PluginManager` to add plugins in a type-safe way.

---

## 6. Quick Reference Cheat Sheet

```
trait Foo { fn bar(&self); }              // contract (no body)
trait Foo { fn bar(&self) { ... } }       // contract with default body

impl Foo for MyType { fn bar(&self) {} }  // implement for one type

fn f<T>(x: T)                             // generic, any type
fn f<T: Foo>(x: T)                        // generic + trait bound
fn f<T>(x: T) where T: Foo                // same, where clause
fn f(x: impl Foo)                         // shorthand for above
fn f() -> impl Foo                        // return, ONE concrete type only
fn f() -> Box<dyn Foo>                    // return, MANY concrete types ok

trait Foo { type Out; fn bar(&self) -> Self::Out; }  // associated type

impl<T: Display> Foo for T { ... }        // blanket impl — for ALL T
```
