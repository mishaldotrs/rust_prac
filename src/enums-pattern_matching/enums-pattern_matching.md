# Enums and Pattern Matching — What Actually Matters

---

## 1. `if let`

Use when you only care about one specific variant and want to ignore the rest.

```rust
let user: Option<String> = Some(String::from("Mishal"));

if let Some(name) = user {
    println!("logged in as {name}");
} else {
    println!("no user found");
}
```

Real backend use — checking optional config or DB result:

```rust
if let Some(user) = db.find_by_id(id).await? {
    return Ok(Json(user));
}
return Err(AppError::NotFound);
```

---

## 2. Destructuring

Unpack data out of enums, structs, and tuples directly in patterns.

```rust
// enum destructuring
enum Event {
    UserCreated { id: u32, name: String },
    UserDeleted(u32),
}

match event {
    Event::UserCreated { id, name } => println!("created {name} with id {id}"),
    Event::UserDeleted(id)          => println!("deleted user {id}"),
}

// struct destructuring
struct Config { host: String, port: u16 }

let Config { host, port } = config;
println!("connecting to {host}:{port}");

// tuple destructuring
let (status, message) = get_response();
println!("{status}: {message}");
```

---

## 3. State Machine

Model every valid state of a system as an enum variant.
Invalid transitions become compile-time errors — not runtime bugs.

```rust
#[derive(Debug)]
enum OrderStatus {
    Pending,
    Confirmed,
    Shipped { tracking_id: String },
    Delivered,
    Cancelled(String),
}

impl OrderStatus {
    fn confirm(self) -> Result<Self, String> {
        match self {
            OrderStatus::Pending => Ok(OrderStatus::Confirmed),
            other => Err(format!("{other:?} cannot be confirmed")),
        }
    }

    fn ship(self, tracking_id: String) -> Result<Self, String> {
        match self {
            OrderStatus::Confirmed => Ok(OrderStatus::Shipped { tracking_id }),
            other => Err(format!("{other:?} cannot be shipped")),
        }
    }

    fn cancel(self, reason: String) -> Result<Self, String> {
        match self {
            OrderStatus::Delivered => Err(String::from("delivered orders cannot be cancelled")),
            _                      => Ok(OrderStatus::Cancelled(reason)),
        }
    }
}
```

Why this matters: you can never accidentally ship a `Pending` order — the compiler stops you.

---

## 4. Result Handling

`Result<T, E>` combined with enums and pattern matching is the core of all error handling in Rust.

```rust
#[derive(Debug)]
enum AppError {
    NotFound,
    Unauthorized,
    BadRequest(String),
    Database(String),
}

fn get_user(id: u32, is_admin: bool) -> Result<String, AppError> {
    if !is_admin {
        return Err(AppError::Unauthorized);
    }

    if id == 0 {
        return Err(AppError::BadRequest(String::from("id cannot be zero")));
    }

    // simulate DB
    match id {
        1 => Ok(String::from("Mishal")),
        _ => Err(AppError::NotFound),
    }
}

fn main() {
    match get_user(1, true) {
        Ok(name)                         => println!("user: {name}"),
        Err(AppError::NotFound)          => println!("404 - not found"),
        Err(AppError::Unauthorized)      => println!("401 - unauthorized"),
        Err(AppError::BadRequest(msg))   => println!("400 - {msg}"),
        Err(AppError::Database(msg))     => println!("500 - {msg}"),
    }
}
```

---

## 5. API Design — How Enums Power Everything

In a real Axum backend, enums and pattern matching are the backbone of the entire request/response cycle.

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

// every possible API failure as an enum
#[derive(Debug)]
enum AppError {
    NotFound,
    Unauthorized,
    BadRequest(String),
    Internal,
}

// enum → HTTP response via pattern matching
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound          => (StatusCode::NOT_FOUND, String::from("not found")),
            AppError::Unauthorized      => (StatusCode::UNAUTHORIZED, String::from("unauthorized")),
            AppError::BadRequest(msg)   => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal          => (StatusCode::INTERNAL_SERVER_ERROR, String::from("server error")),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

// handler — if let + ? + Result + destructuring all together
async fn get_user(id: u32) -> Result<Json<serde_json::Value>, AppError> {
    let user = find_user(id).ok_or(AppError::NotFound)?;

    if let Some(ban_reason) = user.ban_reason {
        return Err(AppError::Unauthorized);
    }

    Ok(Json(json!({ "id": user.id, "name": user.name })))
}
```

---

## How All 5 Connect in Production

```
API Request comes in
        ↓
Handler uses if let      → check if optional value exists
        ↓
Destructuring            → unpack request data cleanly
        ↓
State machine            → validate business logic transitions
        ↓
Result handling          → propagate errors with ?
        ↓
AppError enum            → match each variant to HTTP status
        ↓
HTTP Response sent out
```

These 5 are not separate topics — they are one connected pattern
that every Axum backend is built on.
