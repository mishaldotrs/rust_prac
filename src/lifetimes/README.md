# Lifetimes in Rust

---

## What Are They?

A lifetime is Rust's way of tracking **how long a reference is valid**. Lifetimes don't change how long a value actually lives — they only describe, to the compiler, the relationship between references so it can catch dangling references at compile time.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` here says: *"the returned reference will live at least as long as both `x` and `y`."*

---

## Why Do We Need Them?

Without lifetime tracking, this kind of bug would be possible:

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s   // s is dropped at the end of this function
}        // returned reference would point to freed memory!
```

Rust's borrow checker uses lifetimes to **reject this at compile time**, before the program ever runs — this is exactly what prevents the "dangling reference" problem from the Borrowing rules you already learned in Ownership & Borrowing.

```
Borrowing Rule #2: References must always be valid (no dangling references)
                            ↓
                    Lifetimes are HOW the compiler enforces this rule
```

---

## Topics

### Junior Level

| # | Topic | What it is |
|---|-------|-----------|
| 1 | Why lifetimes exist | Preventing dangling references at compile time |
| 2 | Lifetime annotation syntax `'a` | How to label a reference's lifetime |
| 3 | Lifetime elision rules | When Rust can infer lifetimes automatically |
| 4 | Lifetimes in function signatures | Connecting input and output reference lifetimes |

### Intermediate Level

| # | Topic | What it is |
|---|-------|-----------|
| 5 | Lifetimes in structs | Storing references inside a struct |
| 6 | Multiple lifetime parameters | When two references have independent lifetimes |
| 7 | `'static` lifetime | References valid for the entire program duration |
| 8 | Lifetimes in methods (`impl` blocks) | How lifetime elision works with `&self` |

### Advanced Level

| # | Topic | What it is |
|---|-------|-----------|
| 9  | Lifetime bounds on generics | `T: 'a` — combining lifetimes with generic types |
| 10 | Lifetime subtyping / variance | Why `&'long T` can be used where `&'short T` is expected |
| 11 | Higher-Ranked Trait Bounds (HRTB) | `for<'a>` — lifetimes that work for any lifetime |
| 12 | Lifetimes with closures | Why closures capturing references need careful lifetime handling |

---

## Production Usage in Axum Backend

| Topic | Usage % | Where Used |
|-------|---------|-----------|
| Lifetime elision (invisible, automatic) | 100% | Every function with references — you rarely write `'a` explicitly |
| Explicit `'a` in function signatures | 40% | Custom parsers, string-processing utilities |
| Lifetimes in structs | 45% | Custom extractors, zero-copy parsing, borrowed config |
| `'static` | 60% | `tokio::spawn` requires `'static`, global config, string literals |
| Multiple lifetime parameters | 20% | Complex parsing functions with unrelated reference inputs |
| HRTB (`for<'a>`) | 10% | Closures and trait bounds — usually inferred, rarely written manually |
| Lifetime bounds on generics (`T: 'a`) | 15% | Generic structs holding references |

---

## How This Connects to What You Already Know

```
Ownership & Borrowing  →  Borrowing Rule #2 (no dangling references)
                              ↓
                    Lifetimes are the ENFORCEMENT MECHANISM for that rule

Traits & Generics      →  Generic type parameters <T>
                              ↓
                    Lifetimes are a special kind of generic parameter: <'a>

Async / Tokio (upcoming) →  tokio::spawn requires captured data to be 'static
                              ↓
                    Understanding 'static now prevents confusion later
```

---

## Learning Order

```
Why lifetimes exist (the dangling reference problem)
    ↓
Lifetime annotation syntax ('a)
    ↓
Lifetime elision rules (when you DON'T need to write 'a)
    ↓
Lifetimes in function signatures
    ↓
Lifetimes in structs
    ↓
Multiple lifetime parameters
    ↓
'static lifetime
    ↓
Lifetimes in impl blocks / methods
    ↓
Advanced: lifetime bounds, HRTB, closures with lifetimes
```
