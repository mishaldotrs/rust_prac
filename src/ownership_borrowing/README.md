# Ownership and Borrowing in Rust

---

## What Is It?

Ownership is Rust's core memory management system.
Instead of a garbage collector or manual memory management, Rust uses a set of rules enforced at compile time.
If any rule is violated, the program does not compile.

```rust
let s1 = String::from("hello");
let s2 = s1;          // s1 is moved — ownership transferred
println!("{}", s1);   // ERROR — s1 no longer valid
```

---

## Why Does Rust Have This?

| Problem in other languages | How Rust solves it |
|----------------------------|--------------------|
| Memory leaks (C/C++) | Value is dropped automatically when owner goes out of scope |
| Dangling pointers (C/C++) | References must always be valid — compiler enforces this |
| Data races (Go, Java) | At most one mutable reference at a time — compile-time rule |
| Double free (C/C++) | Only one owner — only one drop |
| Null pointer crashes | No null — use `Option<T>` instead |

---

## Ownership Rules

```
1. Every value has exactly one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.
```

---

## Borrowing Rules

```
1. At any given time you can have either:
      one mutable reference   (&mut T)
   OR any number of immutable references (&T)
   — but NOT both at the same time.

2. References must always be valid.
   (No dangling references allowed.)
```

---

## Topics

### Junior Level

| # | Topic | What it is |
|---|-------|-----------|
| 1 | Ownership basics | Every value has one owner, dropped when owner leaves scope |
| 2 | Move semantics | Ownership transfers when assigned or passed to a function |
| 3 | Copy types | Primitive types are copied, not moved |
| 4 | Clone | Explicitly duplicate a value — deep copy |
| 5 | Immutable references `&T` | Borrow a value without taking ownership — read only |
| 6 | Mutable references `&mut T` | Borrow a value with permission to change it |
| 7 | Dangling references | References that outlive their data — compiler prevents these |

### Intermediate Level

| # | Topic | What it is |
|---|-------|-----------|
| 8  | NLL — Non-Lexical Lifetimes | Borrow ends at last use, not end of block |
| 9  | Slices `&str`, `&[T]` | References to a contiguous part of a collection |
| 10 | Ownership in functions | Passing and returning ownership |
| 11 | Borrowing in functions | Passing references to avoid moving |
| 12 | The borrow checker | Compiler system that enforces all ownership rules |

### Advanced Level

| # | Topic | What it is |
|---|-------|-----------|
| 13 | Lifetimes `'a` | Annotate how long references are valid |
| 14 | Lifetime elision | Rules that let you omit lifetime annotations |
| 15 | `'static` lifetime | Reference valid for the entire program duration |
| 16 | Interior mutability | Mutate through a shared reference — `RefCell`, `Mutex` |
| 17 | `Rc<T>` and `Arc<T>` | Multiple ownership — single thread and multi-thread |
| 18 | `Copy` and `Clone` traits | How types opt in to copy or clone semantics |

---

## Production Usage in Axum Backend

| Topic | Usage % | Where Used |
|-------|---------|-----------|
| Immutable references `&T` | 100% | Handler params, function args everywhere |
| Mutable references `&mut T` | 90% | Modifying state, building responses |
| Move semantics | 90% | Passing ownership into async tasks, spawning threads |
| `Clone` | 85% | `AppState` must be `Clone` for Axum |
| Borrowing in functions | 85% | Avoiding unnecessary copies in hot paths |
| Slices | 75% | String parsing, reading request bodies |
| NLL | 70% | Compiler handles this — you benefit automatically |
| `Arc<T>` | 70% | Shared state across handlers — `Arc<AppState>` |
| Lifetimes | 50% | Custom extractors, middleware, generic functions |
| `'static` | 45% | Spawning tokio tasks — `T: Send + 'static` |
| Interior mutability | 40% | `Arc<Mutex<T>>` for shared mutable state |
| `Rc<T>` | 5% | Almost never in async — use `Arc<T>` instead |

---

## Learning Order

```
Ownership basics
    move semantics
    Copy types
    Clone
        ↓
Borrowing
    immutable references
    mutable references
    dangling references
    NLL
        ↓
Functions
    ownership in functions
    borrowing in functions
    slices
        ↓
Advanced
    lifetimes
    lifetime elision
    'static
    interior mutability
    Rc / Arc
```
