# Error Handling in Rust

---------------------------------------------------------------------------------------------------------

## 🟢 Junior

| # | Concept | Kya karta hai |
|---|---------|---------------|
| 1 | `panic!` | Program crash kar do — unrecoverable error |
| 2 | `Option<T>` | Value hai ya nahi — `Some(val)` ya `None` |
| 3 | `Result<T, E>` | Success ya error — `Ok(val)` ya `Err(e)` |
| 4 | `unwrap()` | Result/Option se value nikalo — panic karta hai if error |
| 5 | `expect()` | unwrap + apna custom message |
| 6 | `match` | Result/Option ko properly handle karo |

---------------------------------------------------------------------------------------------------------

## 🟡 Intermediate

| # | Concept | Kya karta hai |
|---|---------|---------------|
| 7  | `?` operator | Error propagate karo — shorthand for match + return |
| 8  | Custom errors | Apna error type banao — struct ya enum |
| 9  | `Display` for Error | Error ka readable message define karo |
| 10 | `From` trait | Ek error type se doosra automatically convert karo |

---------------------------------------------------------------------------------------------------------

## 🔴 Advanced

| # | Concept | Kya karta hai |
|---|---------|---------------|
| 11 | `Box<dyn Error>` | Multiple alag error types ek saath handle karo |
| 12 | `thiserror` crate | Custom errors easily banao using derive macro |
| 13 | `anyhow` crate | Quick flexible error handling — production apps |
| 14 | Axum errors | `IntoResponse` implement karo apne error pe — backend |

---------------------------------------------------------------------------------------------------------

## Learning Order

```
🟢 Junior
      panic!
      Option<T>       →  Some / None
      Result<T, E>    →  Ok / Err
      unwrap()
      expect()
      match
          ↓
🟡 Intermediate
      ? operator
      Custom errors
      Display for Error
      From trait
          ↓
🔴 Advanced
      Box<dyn Error>
      thiserror
      anyhow
      Axum errors
```

---------------------------------------------------------------------------------------------------------

## Quick Reference

```rust
// Option
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

// Result
let a: Result<i32, String> = Ok(10);
let b: Result<i32, String> = Err(String::from("something went wrong"));

// unwrap — risky
let val = a.unwrap();

// expect — risky but better message
let val = a.expect("failed to get value");

// match — safe
match b {
    Ok(v)  => println!("got: {}", v),
    Err(e) => println!("error: {}", e),
}

// ? operator — propagate error
fn do_something() -> Result<i32, String> {
    let val = might_fail()?;  // agar Err hai toh return kar do
    Ok(val)
}
```
