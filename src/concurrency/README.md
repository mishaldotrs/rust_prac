# Concurrency in Rust

---

## What Is Concurrency?

Running multiple tasks **at the same time** — or appearing to.

```
Without concurrency:
task1 → task2 → task3 → done   (one at a time)

With concurrency:
task1 ──────────────────→
task2 ────────→
task3 ──────────────────────→   (all running simultaneously)
```

---

## Why Rust Is Special For Concurrency

Most languages allow concurrency but let bugs slip through at runtime:
- Data races
- Deadlocks
- Use after free

Rust catches most concurrency bugs **at compile time** through:
- Ownership system
- `Send` and `Sync` traits
- Borrow checker

---

## Topics

### Junior Level

| # | Topic | What it is |
|---|-------|-----------|
| 1 | `thread::spawn` | Create a new OS thread |
| 2 | `thread::join` | Wait for a thread to finish |
| 3 | `move` closures | Transfer ownership into a thread |
| 4 | Channels (`mpsc`) | Send data between threads (message passing) |

### Intermediate Level

| # | Topic | What it is |
|---|-------|-----------|
| 5 | `Send` trait | Type can be transferred across threads |
| 6 | `Sync` trait | Type can be shared across threads |
| 7 | `Arc<Mutex<T>>` | Shared mutable state across threads |
| 8 | Deadlocks | What they are and how to avoid them |

### Advanced Level

| # | Topic | What it is |
|---|-------|-----------|
| 9  | `RwLock<T>` | Multiple readers OR one writer |
| 10 | `Atomic` types | Lock-free thread-safe primitives |
| 11 | Thread pool | Reusing threads instead of spawning new ones |
| 12 | `Barrier` | Synchronize multiple threads at a point |

---

## Production Usage in Axum Backend

| Topic | Usage % | Where Used |
|-------|---------|-----------|
| `Send + Sync` | 🔥🔥🔥 100% | Every handler, AppState, shared data |
| `Arc<Mutex<T>>` | 🔥🔥🔥 85% | Shared mutable state across handlers |
| `Arc<RwLock<T>>` | 🔥🔥 65% | Read-heavy shared state (cache, config) |
| Channels (mpsc) | 🔥🔥 60% | Background tasks, event processing |
| `Atomic` types | 🔥🔥 55% | Counters, flags without Mutex overhead |
| Thread pool | 🔥🔥 50% | tokio handles this internally |
| Deadlock avoidance | 🔥🔥🔥 100% | Always need to think about this |

---

## Learning Order

```
thread::spawn + join    ← create threads
move closures           ← send data TO thread
channels (mpsc)         ← send data BETWEEN threads
Send + Sync             ← understand why some types work, others don't
Arc<Mutex<T>>           ← shared mutable state (already know this!)
Deadlocks               ← how to avoid
RwLock                  ← read-heavy optimization
Atomic types            ← lock-free performance
```
