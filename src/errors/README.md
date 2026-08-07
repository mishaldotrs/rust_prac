# Error Handling in Rust

---

## What Is It?

Rust separates failures into two categories:

- **Unrecoverable** — a bug or impossible state. Use `panic!`.
- **Recoverable** — an expected failure. Use `Result<T, E>` or `Option<T>`.

```rust
// Recoverable
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err(String::from("division by zero")) }
    else        { Ok(a / b) }
}

// Unrecoverable
fn get_element(v: &Vec<i32>, i: usize) -> i32 {
    if i >= v.len() { panic!("index out of bounds — bug"); }
    v[i]
}
```

---

## Why Does Rust Handle Errors This Way?

| Problem | Rust's Solution |
|---------|----------------|
| Silent null pointer crashes | No null — use `Option<T>` |
| Unchecked exceptions (Java) | `Result<T, E>` must be handled — compiler enforces it |
| Error type confusion | Typed error enums — each variant is a specific failure |
| Hidden failures | `?` operator makes propagation explicit and visible |
| Verbose error boilerplate | `thiserror` and `anyhow` reduce repetition |

---

## Topics

### Junior Level

| # | Topic | What it is |
|---|-------|-----------|
| 1 | `panic!` | Stop execution — unrecoverable error |
| 2 | `Option<T>` | Value may be present `Some` or absent `None` |
| 3 | `Result<T, E>` | Success `Ok` or failure `Err` |
| 4 | `unwrap()` | Extract value or panic |
| 5 | `expect()` | Extract value or panic with a message |
| 6 | `match` | Handle every variant explicitly |

### Intermediate Level

| # | Topic | What it is |
|---|-------|-----------|
| 7  | `?` operator | Propagate error early — shorthand for match + return |
| 8  | Custom errors | Model domain failures as enum variants |
| 9  | `Display` + `Error` | Give errors readable messages |
| 10 | `From` trait | Convert one error type to another automatically |

### Advanced Level

| # | Topic | What it is |
|---|-------|-----------|
| 11 | `Box<dyn Error>` | Multiple error types behind one interface |
| 12 | `thiserror` | Typed errors with derive macro |
| 13 | `anyhow` | Flexible errors with context for application code |
| 14 | Axum errors | `IntoResponse` — convert errors into HTTP responses |

---

## Production Usage in Axum Backend

| Topic | Usage % | Where Used |
|-------|---------|-----------|
| `Result<T, E>` | 100% | Every handler, service, repository |
| `?` operator | 95% | Every function that calls fallible operations |
| Custom error enum | 90% | `AppError`, `AuthError`, `DbError` |
| `IntoResponse` for errors | 90% | Every handler error response |
| `thiserror` | 80% | Domain error types |
| `Display` + `Error` | 80% | Logging, response messages |
| `From` trait | 75% | Converting `sqlx::Error` → `AppError` |
| `Option<T>` | 70% | DB lookups — user not found |
| `anyhow` | 65% | Service layer, internal tooling |
| `match` | 60% | Specific variant handling |
| `expect()` | 15% | Startup — env vars, config |
| `unwrap()` | 10% | Tests only |
| `panic!` | 5% | Startup assertions only |

---

## Learning Order

```
panic!
Option<T>
Result<T, E>
unwrap / expect
match
    ↓
? operator
custom error enums
Display + Error trait
From trait
    ↓
Box<dyn Error>
thiserror
anyhow
Axum IntoResponse errors
```
