# Enums and Pattern Matching in Rust

---

## What Are They?

**Enum** — a type that can be one of several named variants, each optionally holding data.

```rust
enum Status {
    Active,
    Inactive,
    Banned(String),
}
```

**Pattern Matching** — destructuring and inspecting values at runtime, exhaustively.

```rust
match status {
    Status::Active       => println!("active"),
    Status::Inactive     => println!("inactive"),
    Status::Banned(msg)  => println!("banned: {msg}"),
}
```

---

## Why Do We Use Them?

**Without enums:**

```rust
// stringly typed — error prone
if status == "active" { }
if status == "Actve"  { }  // typo — no compile error
```

**With enums:**

```rust
// compiler catches every missing case and every typo
match status {
    Status::Active   => { }
    Status::Inactive => { }
    // forget Banned? compile error.
}
```

Enums make impossible states unrepresentable at compile time.

---

## Topics

| # | Topic | What it is |
|---|-------|-----------|
| 1 | `if let` | Match one variant, ignore the rest |
| 2 | Destructuring | Unpack data from enums, structs, tuples in patterns |
| 3 | State machine | Model valid states and transitions as enum variants |
| 4 | Result handling | `Result<T, E>` with enum error variants and pattern matching |
| 5 | API design | How enums power the entire Axum request/response cycle |

---

## Production Usage in Axum Backend

| Topic | Usage % | Where Used |
|-------|---------|-----------|
| `Result<T, E>` | 100% | Every handler and service return type |
| `Option<T>` | 95% | DB lookups, optional fields |
| `match` | 90% | Error handling, routing logic, state transitions |
| Enum with data | 90% | `AppError`, request/response types, domain models |
| `if let` | 85% | Checking optional config, extracting values |
| `#[derive]` on enums | 85% | Debug, Clone, Serialize, Deserialize |
| Enum with trait impl | 80% | `IntoResponse` for `AppError` |
| State machine | 70% | Order status, payment flow, user lifecycle |
| Destructuring | 65% | Unpacking request payloads and DB results |

---

## How All Topics Connect

```
API Request comes in
        ↓
if let           → check if optional value exists
        ↓
Destructuring    → unpack request data cleanly
        ↓
State machine    → validate business logic transitions
        ↓
Result handling  → propagate errors with ?
        ↓
AppError enum    → match each variant to HTTP status
        ↓
HTTP Response sent out
```

---

## Learning Order

```
Enum basics
    match
    if let
    while let
        ↓
Destructuring
    enums with data
    struct fields
    tuples
        ↓
Patterns
    match guards
    OR patterns
    @ bindings
        ↓
Production patterns
    state machine
    result handling
    API error design
```
