# Traits and Generics in Rust

---

## What Are They?

**Trait** — a contract that defines what a type can do.

```rust
trait MakeSound {
    fn sound(&self) -> String;
}
```

**Generic** — a placeholder type `<T>` decided by the caller.

```rust
fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

---

## Why Do We Use Them?

**Without traits and generics:**

```rust
fn print_i32(x: i32)   { println!("{}", x); }
fn print_f64(x: f64)   { println!("{}", x); }
fn print_str(x: &str)  { println!("{}", x); }
// same code, three times — not scalable
```

**With generics:**

```rust
fn print_it<T: std::fmt::Display>(x: T) {
    println!("{}", x);
}
// works for i32, f64, &str, String — any Display type
```

**Without traits:**

```rust
fn dog_sound()   -> String { String::from("Woof") }
fn cat_sound()   -> String { String::from("Meow") }
// cannot treat Dog and Cat uniformly
```

**With traits:**

```rust
fn make_noise(animal: &dyn MakeSound) {
    println!("{}", animal.sound());
}
// works for any type that implements MakeSound
```

---

## Topics

### Junior Level

| # | Topic | What it is |
|---|-------|-----------|
| 1 | Trait basics | Contract — defines what a type must be able to do |
| 2 | Trait impl | Fulfil the contract for a specific struct or enum |
| 3 | Default impl | Provide a fallback body — overriding is optional |
| 4 | Generic `<T>` | Placeholder type decided by the caller |
| 5 | Trait bounds | Restrict T — `T: SomeTrait` |
| 6 | `impl Trait` | Shorthand for trait bounds in parameters and return types |
| 7 | `dyn Trait` | Runtime polymorphism — type decided at runtime |
| 8 | Associated types | A type declared inside a trait, set by the implementor |
| 9 | Blanket impl | One generic impl that applies to all qualifying types |

### Advanced Level

| # | Topic | What it is |
|---|-------|-----------|
| 10 | Supertraits | A trait that requires another trait first |
| 11 | Marker traits | No methods — just a tag (`Send`, `Sync`, `Copy`) |
| 12 | Object safety | Rules for when a trait can be used as `dyn Trait` |
| 13 | Const generics | Generic over a value — `fn foo<const N: usize>()` |
| 14 | `move` closures with generics | Force ownership capture in generic contexts |

---

## Production Usage in Axum Backend

| Topic | Usage % | Where Used |
|-------|---------|-----------|
| Trait basics + impl | 100% | Repository, service, error traits |
| `impl Trait` | 100% | `-> impl IntoResponse` on every handler |
| Trait bounds | 95% | `T: Send + Sync + Clone + 'static` everywhere |
| Generic `<T>` | 95% | `State<T>`, `Json<T>`, `AppState<T>` |
| `dyn Trait` | 90% | `Arc<dyn UserRepository>` dependency injection |
| Marker traits | 90% | `Send + Sync + Clone` on AppState |
| Associated types | 80% | `Iterator::Item`, `FromRequest` |
| Blanket impl | 35% | Understanding std library behaviour |
| Supertraits | 50% | Some Axum traits require others |
| Object safety | 75% | Needed when using `dyn Trait` |

---

## Learning Order

```
Trait basics
    trait impl
    default impl
        ↓
Generics
    generic functions
    generic structs
    trait bounds
    where clause
        ↓
Polymorphism
    impl Trait
    dyn Trait
        ↓
Advanced
    associated types
    blanket impl
    supertraits
    marker traits
```
