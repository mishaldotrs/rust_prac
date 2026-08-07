# Iterators and Closures in Rust

---

## What Are They?

**Closure** — an anonymous function that can capture variables from the scope around it.

```rust
let add = |x, y| x + y;
println!("{}", add(2, 3)); // 5
```

**Iterator** — a type that produces a sequence of values one at a time, on demand.

```rust
let nums = vec![1, 2, 3];
let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
// [2, 4, 6]
```

---

## Why Do We Use Them?

### Without iterators and closures:

```rust
let nums = vec![1, 2, 3, 4, 5];
let mut result = Vec::new();

for n in &nums {
    if n % 2 == 0 {
        result.push(n * 10);
    }
}
```

### With iterators and closures:

```rust
let result: Vec<i32> = nums.iter()
    .filter(|n| *n % 2 == 0)
    .map(|n| n * 10)
    .collect();
```

Same result — but cleaner, shorter, and more expressive.

---

## Why Rust Specifically Pushes You Toward Them

**1. Zero cost abstractions**
Iterator chains compile down to the same machine code as a hand-written loop.
You get readable high-level code with no runtime performance penalty.

**2. No manual index management**
No off-by-one errors, no `i < len` bugs, no out-of-bounds panics from manual indexing.

**3. Lazy evaluation**
Iterators do not compute anything until you call a consuming method like `.collect()` or `.for_each()`.
This means chaining ten operations costs no more than one pass through the data.

**4. Works with the ownership system**
`.iter()` borrows, `.iter_mut()` mutably borrows, `.into_iter()` takes ownership.
The compiler enforces correct usage automatically.

**5. Used everywhere in production**
Every Axum handler, every database query result, every config parser — all use iterator chains.

---

## Topics

### Junior Level

| # | Topic | What it is |
|---|-------|-----------|
| 1 | Closure basics | Anonymous function — `\|x\| x + 1` |
| 2 | Capturing variables | Closures borrow or move variables from outer scope |
| 3 | `iter()`, `iter_mut()`, `into_iter()` | Three ways to iterate a collection |
| 4 | `.map()` | Transform each element |
| 5 | `.filter()` | Keep only elements that match a condition |
| 6 | `.collect()` | Consume the iterator into a collection |
| 7 | `.for_each()` | Run a closure on each element, return nothing |
| 8 | `.count()`, `.sum()`, `.max()`, `.min()` | Reduce the iterator to a single value |

### Intermediate Level

| # | Topic | What it is |
|---|-------|-----------|
| 9  | `.chain()` | Join two iterators together |
| 10 | `.enumerate()` | Pair each element with its index |
| 11 | `.zip()` | Pair elements from two iterators |
| 12 | `.flat_map()` | Map then flatten nested iterators |
| 13 | `.take()` and `.skip()` | Limit how many elements you process |
| 14 | `.find()` and `.position()` | Search for an element |
| 15 | `.any()` and `.all()` | Check a condition across the whole iterator |
| 16 | `.fold()` | Accumulate a value across all elements |
| 17 | `Fn`, `FnMut`, `FnOnce` traits | The three closure trait bounds |
| 18 | Returning closures from functions | `impl Fn` and `Box<dyn Fn>` |

### Advanced Level

| # | Topic | What it is |
|---|-------|-----------|
| 19 | Custom `Iterator` trait impl | Build your own iterator type |
| 20 | `.peekable()` | Look at the next element without consuming it |
| 21 | `.windows()` and `.chunks()` | Sliding and fixed-size slices |
| 22 | Iterator adapters as arguments | Accept `impl Iterator<Item = T>` in functions |
| 23 | `move` closures | Force ownership capture — required for threads and async |
| 24 | Closures in structs | Store closures as fields using `Box<dyn Fn>` or generics |

---

## Production Usage in Axum Backend

| Topic | Usage % | Where Used |
|-------|---------|-----------|
| `.map()` | 100% | Transform DB rows → response structs |
| `.filter()` | 95% | Filter query results, validate inputs |
| `.collect()` | 95% | Turn iterator results into Vec, HashMap |
| `move` closures | 90% | Tokio tasks, async handlers, spawned threads |
| `Fn` / `FnMut` / `FnOnce` | 85% | Middleware, callbacks, handler generics |
| `.for_each()` | 80% | Side effects — logging, sending events |
| `.enumerate()` | 75% | Index-aware processing of results |
| `.fold()` | 70% | Aggregating totals, building strings |
| `.flat_map()` | 65% | Flattening nested DB results |
| `.any()` / `.all()` | 65% | Permission checks, validation |
| `.find()` | 60% | Searching in a collection |
| `Box<dyn Fn>` | 55% | Storing callbacks, middleware chains |
| Custom Iterator | 30% | Pagination, streaming responses |
| `.peekable()` | 20% | Parsers, tokenizers |

---

## How They Connect to What You Already Know

```
Traits & Generics     →  Iterator is a trait with associated type Item
                          Fn, FnMut, FnOnce are traits for closures
                          .map(|x| x+1) takes impl FnMut as argument

Enums & Pattern Match →  .map() and .filter() use closures
                          Option returned by .find(), .next()
                          Result returned by .collect::<Result<Vec<_>, _>>()

Error Handling        →  iterator chains can propagate errors
                          .map(|x| parse(x)?).collect::<Result<Vec<_>, _>>()
```

---

## Learning Order

```
Closures
    basics — |x| x + 1
    capturing variables
    Fn / FnMut / FnOnce
        ↓
Iterator basics
    iter() / iter_mut() / into_iter()
    map / filter / collect
    for_each / sum / count
        ↓
Intermediate
    chain / enumerate / zip
    flat_map / fold
    find / any / all
    take / skip
        ↓
Advanced
    Custom Iterator impl
    move closures
    Closures in structs
    Iterator adapters as fn args
```
