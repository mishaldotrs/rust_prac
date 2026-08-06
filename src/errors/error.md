# Error Handling in Rust

Rust separates failures into two broad categories:

- **Unrecoverable errors** indicate a bug or an invalid program state. They are usually handled with `panic!`.
- **Recoverable errors** are expected failures such as invalid input, a missing file, or a failed network request. They are represented with `Option<T>` or `Result<T, E>`.

## Junior Level

| # | Concept | Purpose |
|---|---|---|
| 1 | `panic!` | Stop execution when the program reaches an unrecoverable or impossible state. |
| 2 | `Option<T>` | Represent a value that may be present (`Some`) or absent (`None`). |
| 3 | `Result<T, E>` | Represent either success (`Ok`) or a recoverable failure (`Err`). |
| 4 | `unwrap()` | Extract a value, but panic if it is `None` or `Err`. |
| 5 | `expect()` | Extract a value like `unwrap`, but use a custom panic message on failure. |
| 6 | `match` | Explicitly handle every possible `Option` or `Result` variant. |

## Intermediate Level

| # | Concept | Purpose |
|---|---|---|
| 7 | `?` operator | Return an error early while keeping error-handling code concise. |
| 8 | Custom error types | Model the specific failures that can occur in your application. |
| 9 | `Display` and `Error` | Give a custom error a readable message and make it a standard Rust error. |
| 10 | `From` trait | Convert one error type into another automatically, especially when using `?`. |

## Advanced Level

| # | Concept | Purpose |
|---|---|---|
| 11 | `Box<dyn Error>` | Return different standard error types through one dynamic error type. |
| 12 | `thiserror` | Define typed library or application errors with less boilerplate. |
| 13 | `anyhow` | Add context and propagate flexible errors in application code. |
| 14 | Axum errors | Convert application errors into appropriate HTTP responses. |

---

# Detailed Explanations and Examples

## 1. `panic!` — Unrecoverable Errors

`panic!` immediately stops normal execution because the program has entered a state from which it should not continue.

Use it when:

- An internal invariant has been broken.
- Continuing could produce corrupted or misleading results.
- A condition represents a programming bug rather than a normal runtime failure.
- Writing tests and assertions.

Do not normally use it for invalid user input, missing files, database failures, or network failures. Those are expected failures and should return `Result`.

```rust
fn set_percentage(value: u8) {
    if value > 100 {
        panic!("percentage cannot be greater than 100");
    }

    println!("Percentage: {value}%");
}

fn main() {
    set_percentage(80);
    set_percentage(120); // The program panics here.
}
```

The main problem solved by `panic!` is **fail-fast behavior**: the program does not silently continue after detecting an invalid internal state.

---

## 2. `Option<T>` — A Value May Be Missing

`Option<T>` is not an error by itself. It represents the possibility that a value does not exist.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example:

```rust
fn find_user_name(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Mishal"))
    } else {
        None
    }
}

fn main() {
    match find_user_name(1) {
        Some(name) => println!("User: {name}"),
        None => println!("User was not found"),
    }
}
```

Use `Option<T>` when absence is meaningful but you do not need an explanation for why the value is absent.

---

## 3. `Result<T, E>` — Recoverable Success or Failure

`Result<T, E>` represents an operation that can succeed with `T` or fail with `E`.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Example:

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("the denominator cannot be zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("{:?}", divide(10.0, 2.0)); // Ok(5.0)
    println!("{:?}", divide(10.0, 0.0)); // Err(...)
}
```

Use `Result` when the caller should be able to inspect, handle, log, retry, or propagate the failure.

---

## 4. `unwrap()` — Extract or Panic

`unwrap()` returns the inner value from `Some` or `Ok`. It panics for `None` or `Err`.

```rust
fn main() {
    let valid_number = "42".parse::<i32>();
    let number = valid_number.unwrap();
    println!("{number}");

    let invalid_number = "hello".parse::<i32>();
    let _number = invalid_number.unwrap(); // Panics.
}
```

Appropriate uses include:

- Small experiments.
- Tests.
- Situations where success has already been logically guaranteed.

Avoid `unwrap()` on user input, network operations, database operations, and other production failure points.

---

## 5. `expect()` — Extract or Panic With Context

`expect()` behaves like `unwrap()`, but its custom message explains why the value was expected to exist.

```rust
use std::fs;

fn main() {
    let config = fs::read_to_string("config.toml")
        .expect("config.toml must exist before the application starts");

    println!("{config}");
}
```

If panicking is genuinely appropriate, `expect()` is normally better than `unwrap()` because its message makes debugging easier.

A good message describes the assumption that was violated, not merely that an operation failed.

---

## 6. `match` — Explicit Error Handling

`match` forces you to handle both success and failure paths.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 0.0) {
        Ok(value) => println!("Result: {value}"),
        Err(error) => println!("Could not divide: {error}"),
    }
}
```

Use `match` when the current function can meaningfully recover—for example, by using a fallback value, retrying, returning a different response, or logging the error.

---

## 7. The `?` Operator — Propagate Errors

The `?` operator extracts the success value. If the value is an error, it converts the error when necessary and returns it immediately from the current function.

```rust
use std::fs;
use std::io;

fn read_username() -> Result<String, io::Error> {
    let username = fs::read_to_string("username.txt")?;
    Ok(username)
}

fn main() -> Result<(), io::Error> {
    let username = read_username()?;
    println!("Username: {username}");
    Ok(())
}
```

This:

```rust
let username = fs::read_to_string("username.txt")?;
```

is conceptually similar to:

```rust
let username = match fs::read_to_string("username.txt") {
    Ok(value) => value,
    Err(error) => return Err(error),
};
```

Use `?` when the current layer cannot handle the error and the caller should decide what to do.

---

## 8. Custom Error Types — Model Application Failures

A custom error enum describes the failures that belong to your domain.

```rust
#[derive(Debug)]
enum RegistrationError {
    EmptyName,
    InvalidAge(u8),
    EmailAlreadyExists(String),
}

fn register(name: &str, age: u8, email_exists: bool) -> Result<(), RegistrationError> {
    if name.trim().is_empty() {
        return Err(RegistrationError::EmptyName);
    }

    if age < 18 {
        return Err(RegistrationError::InvalidAge(age));
    }

    if email_exists {
        return Err(RegistrationError::EmailAlreadyExists(
            String::from("user@example.com"),
        ));
    }

    Ok(())
}
```

Custom errors are better than returning arbitrary strings because callers can match individual variants safely.

```rust
match register("Mishal", 16, false) {
    Ok(()) => println!("Registered"),
    Err(RegistrationError::InvalidAge(age)) => {
        println!("Age {age} is below the minimum")
    }
    Err(error) => println!("Other registration error: {error:?}"),
}
```

---

## 9. Implementing `Display` and `Error`

`Debug` is intended for developers. `Display` provides a readable error message for users, logs, and responses. Implementing `std::error::Error` makes the type compatible with the standard error ecosystem.

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum PaymentError {
    InvalidAmount,
    InsufficientFunds,
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaymentError::InvalidAmount => write!(f, "the payment amount is invalid"),
            PaymentError::InsufficientFunds => write!(f, "insufficient funds"),
        }
    }
}

impl Error for PaymentError {}

fn charge(balance: u32, amount: u32) -> Result<u32, PaymentError> {
    if amount == 0 {
        return Err(PaymentError::InvalidAmount);
    }

    if amount > balance {
        return Err(PaymentError::InsufficientFunds);
    }

    Ok(balance - amount)
}
```

Now `PaymentError` can be printed with `{}` and used as a standard error.

---

## 10. The `From` Trait — Automatic Error Conversion

`From` converts one error type into another. The `?` operator uses this conversion automatically.

```rust
use std::fs;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

fn read_port() -> Result<u16, AppError> {
    let text = fs::read_to_string("port.txt")?;
    let port = text.trim().parse::<u16>()?;
    Ok(port)
}
```

The first `?` converts `io::Error` into `AppError`. The second converts `ParseIntError` into `AppError`.

In a complete application, `AppError` should also implement `Display` and `Error`.

---

## 11. `Box<dyn Error>` — Different Error Types Behind One Interface

`Box<dyn Error>` is a boxed trait object. It lets a function return different concrete error types as long as they implement `std::error::Error`.

```rust
use std::error::Error;
use std::fs;

fn read_port() -> Result<u16, Box<dyn Error>> {
    let text = fs::read_to_string("port.txt")?; // io::Error
    let port = text.trim().parse::<u16>()?;      // ParseIntError
    Ok(port)
}

fn main() -> Result<(), Box<dyn Error>> {
    let port = read_port()?;
    println!("Port: {port}");
    Ok(())
}
```

Advantages:

- Convenient for small applications, scripts, and prototypes.
- Supports multiple error types without defining a custom enum.

Trade-offs:

- The exact error variants are erased behind dynamic dispatch.
- Callers cannot easily match a strongly typed application error enum.

For reusable libraries, a concrete custom error type is usually better.

---

## 12. `thiserror` — Typed Errors With Less Boilerplate

`thiserror` generates implementations of `Display`, `Error`, and `From` for custom error types.

Add the dependency:

```toml
[dependencies]
thiserror = "2"
```

Example:

```rust
use std::fs;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("failed to read the configuration file")]
    Io(#[from] std::io::Error),

    #[error("port must be a valid u16")]
    InvalidPort(#[from] ParseIntError),
}

fn read_port() -> Result<u16, ConfigError> {
    let text = fs::read_to_string("port.txt")?;
    let port = text.trim().parse::<u16>()?;
    Ok(port)
}
```

Use `thiserror` when callers need structured, matchable errors. It is especially useful for libraries and domain/application error enums.

---

## 13. `anyhow` — Flexible Application Errors and Context

`anyhow` provides a convenient error type for application code and allows extra context to be attached while propagating errors.

Add the dependency:

```toml
[dependencies]
anyhow = "1"
```

Example:

```rust
use anyhow::{Context, Result};
use std::fs;

fn read_port() -> Result<u16> {
    let text = fs::read_to_string("port.txt")
        .context("failed to read port.txt")?;

    let port = text
        .trim()
        .parse::<u16>()
        .context("port.txt did not contain a valid u16")?;

    Ok(port)
}

fn main() -> Result<()> {
    let port = read_port()?;
    println!("Port: {port}");
    Ok(())
}
```

General guideline:

- Use `thiserror` for errors that callers need to inspect and match.
- Use `anyhow` for executable application code where convenient propagation and context are more important than exposing a stable error API.

---

## 14. Axum Errors — Convert Failures Into HTTP Responses

An Axum handler can return `Result<T, E>`. The success and error types must be convertible into HTTP responses. A common pattern is to implement `IntoResponse` for an application error enum.

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
enum AppError {
    UserNotFound,
    DatabaseFailure,
    InvalidInput(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::UserNotFound => {
                (StatusCode::NOT_FOUND, String::from("user not found"))
            }
            AppError::DatabaseFailure => {
                (StatusCode::INTERNAL_SERVER_ERROR, String::from("internal server error"))
            }
            AppError::InvalidInput(message) => {
                (StatusCode::BAD_REQUEST, message)
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

async fn get_user() -> Result<Json<serde_json::Value>, AppError> {
    let user_exists = false;

    if !user_exists {
        return Err(AppError::UserNotFound);
    }

    Ok(Json(json!({ "id": 1, "name": "Mishal" })))
}
```

This pattern solves an important backend problem: internal Rust errors must be translated into stable HTTP status codes and safe client-facing messages.

Do not expose database credentials, SQL statements, stack traces, or other sensitive internal details in HTTP responses. Log internal details on the server and return a safe generic message to the client.

---

# Recommended Learning Order

```text
1. panic!
2. Option<T>
3. Result<T, E>
4. match
5. unwrap() and expect()
6. ? operator
7. Custom error enums
8. Display and std::error::Error
9. From conversions
10. Box<dyn Error>
11. thiserror
12. anyhow
13. Axum IntoResponse errors
```

# Practical Rules

1. Use `panic!` for programming bugs or impossible internal states.
2. Use `Option<T>` when a value may simply be absent.
3. Use `Result<T, E>` when an operation may fail and the reason matters.
4. Prefer `match`, combinators, or `?` over `unwrap()` in production code.
5. Add context at application boundaries such as configuration, database, network, and file operations.
6. Use typed errors when callers need to make decisions based on specific variants.
7. Translate internal backend errors into safe HTTP responses at the Axum boundary.
