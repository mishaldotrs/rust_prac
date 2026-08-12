# Smart Pointers in Rust

---

## What Are Smart Pointers?

A regular reference (`&T`) just points to data — it has no ownership and no extra behavior.

A **smart pointer** is a struct that:
- Acts like a pointer (you can dereference it)
- Also **owns** the data it points to
- Has **extra behavior** (reference counting, heap allocation, interior mutability)

```rust
let x = 5;
let r = &x;          // regular reference — no ownership
let b = Box::new(5); // smart pointer — OWNS the data on the heap
```

---

## Why Do We Need Them?

| Problem | Smart Pointer Solution |
|---------|----------------------|
| Need heap allocation | `Box<T>` |
| Multiple owners, single thread | `Rc<T>` |
| Multiple owners, multiple threads | `Arc<T>` |
| Mutate through shared reference (single thread) | `RefCell<T>` |
| Mutate through shared reference (multi thread) | `Mutex<T>` |
| Prevent value from moving in memory (async) | `Pin<T>` |

---

## Topics

### Junior Level

| # | Topic | What it is |
|---|-------|-----------|
| 1 | `Box<T>` | Store data on heap, single owner |
| 2 | `Rc<T>` | Multiple owners, single thread, reference counted |
| 3 | `Arc<T>` | Multiple owners, multiple threads, atomic reference counted |

### Intermediate Level

| # | Topic | What it is |
|---|-------|-----------|
| 4 | `RefCell<T>` | Interior mutability — single thread |
| 5 | `Mutex<T>` | Interior mutability — multiple threads |
| 6 | `Arc<Mutex<T>>` | Shared mutable state across threads — Axum pattern |

### Advanced Level

| # | Topic | What it is |
|---|-------|-----------|
| 7 | `Cell<T>` | Interior mutability for Copy types |
| 8 | `Pin<T>` | Prevent moving in memory — async/await |
| 9 | `Cow<T>` | Clone on write — avoid unnecessary cloning |

---

## Production Usage in Axum Backend

| Smart Pointer | Usage % | Where Used |
|---------------|---------|-----------|
| `Arc<T>` | 🔥🔥🔥 95% | Shared AppState across handlers |
| `Mutex<T>` | 🔥🔥🔥 85% | Mutable shared state (counters, caches) |
| `Arc<Mutex<T>>` | 🔥🔥🔥 85% | The most common Axum state pattern |
| `Box<T>` | 🔥🔥 70% | `Box<dyn Error>`, trait objects |
| `Arc<RwLock<T>>` | 🔥🔥 65% | Read-heavy shared state |
| `RefCell<T>` | 🔥 20% | Single-threaded interior mutability |
| `Rc<T>` | 🔥 10% | Almost never in async — use Arc instead |
| `Pin<T>` | 🔥 15% | Async internals — rarely written manually |

---

## How They Connect to What You Already Know

```
Ownership      → Box<T> owns data on heap (single owner)
                 Rc/Arc share ownership (multiple owners)

Borrowing      → RefCell/Mutex allow mutation through shared reference
                 (interior mutability — exception to borrowing rules)

Traits         → Deref trait makes smart pointers work like references
                 Drop trait defines cleanup when smart pointer is dropped

Lifetimes      → Arc<T> satisfies 'static — works with tokio::spawn

Async/Await    → Pin<T> needed for Futures
                 Arc<Mutex<T>> needed for shared state in async handlers
```

---

## Learning Order

```
Box<T>               ← heap allocation, single owner
    ↓
Rc<T>                ← multiple owners, single thread
    ↓
Arc<T>               ← multiple owners, multi-thread (CRITICAL for Axum)
    ↓
RefCell<T>           ← interior mutability, single thread
    ↓
Mutex<T>             ← interior mutability, multi-thread
    ↓
Arc<Mutex<T>>        ← THE Axum pattern — shared mutable state
    ↓
Pin<T>               ← async internals
```
