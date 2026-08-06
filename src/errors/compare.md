# Error Handling — Where to Use What

---

## Production Usage in Axum Backend

| # | Concept | Usage % | Where Used in Axum Backend |
|---|---------|---------|---------------------------|
| 1 | `Result<T, E>` | 100% | Every handler, service, repository — return type everywhere |
| 2 | `?` operator | 95% | Every function that calls fallible operations |
| 3 | Custom error enum | 90% | Domain errors — `UserError`, `AuthError`, `DbError` |
| 4 | `IntoResponse` for errors | 90% | Every handler — convert AppError into HTTP response |
| 5 | `thiserror` | 80% | Defining domain and application error types |
| 6 | `Display` + `Error` trait | 80% | Logging, response messages, chaining errors |
| 7 | `From` trait | 75% | Converting `sqlx::Error` → `AppError`, `io::Error` → `AppError` |
| 8 | `Option<T>` | 70% | DB lookups — user not found, optional fields |
| 9 | `anyhow` | 65% | Service layer, scripts, internal tooling |
| 10 | `match` | 60% | When a specific variant needs different handling |
| 11 | `Box<dyn Error>` | 20% | Rarely — replaced by `thiserror`/`anyhow` in Axum apps |
| 12 | `expect()` | 15% | Startup only — reading env vars, connecting DB on boot |
| 13 | `unwrap()` | 10% | Tests only — never in handler code |
| 14 | `panic!` | 5% | Startup assertions, impossible internal states only |

---

## Which Error to Use — Decision Guide

```
Is the failure a programming bug or truly impossible state?
    YES → panic!
    NO  ↓

Does the value simply not exist (no reason needed)?
    YES → Option<T>
    NO  ↓

Can the caller recover from this failure?
    NO  → panic!
    YES ↓

Are you in a handler / service function?
    YES → return Result<T, AppError> with ?
    NO  ↓

Do you need callers to match specific failure variants?
    YES → thiserror custom enum
    NO  → anyhow
```

---

## Layer by Layer — What Goes Where in Axum

```
HTTP Layer (Axum handlers)
    └── Result<Json<T>, AppError>
    └── AppError implements IntoResponse
    └── ? operator to propagate

Service Layer (business logic)
    └── Result<T, AppError>
    └── thiserror for domain errors
    └── ? operator everywhere
    └── From to convert DB/IO errors into AppError

Repository Layer (database)
    └── Result<T, DbError>
    └── sqlx::Error converted via From into AppError
    └── Option<User> for find_by_id

Startup (main / server init)
    └── expect() for env vars, DB connection
    └── anyhow::Result<()> for main()
```

---

## Axum Core — What Errors Axum Uses Internally

| Axum Internal | What it Does |
|---------------|-------------|
| `axum::Error` | Base error type for Axum internals |
| `JsonRejection` | Request body is not valid JSON |
| `PathRejection` | Path parameter missing or wrong type |
| `QueryRejection` | Query parameter missing or wrong type |
| `FormRejection` | Form data is invalid |
| `MethodNotAllowed` | Wrong HTTP method for the route |
| `FailedToDeserializeQueryString` | Query string cannot be deserialized |
| `MissingExtension` | Extension not added to request |
| `StatusCode` | Used to set HTTP response status |
| `IntoResponse` | Trait every response type must implement |

These are part of `axum::extract::rejection::*`. When you use `Json<T>`, `Path<T>`, or `Query<T>` as extractors, Axum returns these rejection types automatically if the request does not match.

---

## Real Axum App — Error Flow Example

```
Client sends POST /users with invalid JSON body
            ↓
Axum extractor → JsonRejection (Axum internal)
            ↓
Handler never called — Axum returns 422 automatically

Client sends POST /users with valid JSON but age = 15
            ↓
Handler called
            ↓
Service layer: register("Ali", 15) → Err(UserError::AgeTooLow(15))
            ↓
? operator returns Err to handler
            ↓
AppError::User(UserError::AgeTooLow(15))
            ↓
IntoResponse → StatusCode::BAD_REQUEST + JSON body
            ↓
Client receives: { "error": "age 15 is below the minimum of 18" }
```

---

## Quick Comparison — thiserror vs anyhow

| | `thiserror` | `anyhow` |
|--|-------------|---------|
| Purpose | Define typed, matchable errors | Propagate flexible errors with context |
| Callers can match variants | Yes | No |
| Add context to errors | No | Yes — `.context("...")` |
| Use in libraries | Yes | No |
| Use in Axum handlers/services | Yes | Yes (service layer) |
| Boilerplate | Low (derive macro) | None |
| Error type | Concrete named type | Opaque `anyhow::Error` |

**Recommendation for Axum:**
- Define `AppError` with `thiserror` — typed, matchable, converts to HTTP response.
- Use `anyhow` inside service functions where context matters more than variant matching.
- Convert `anyhow::Error` into `AppError` at the handler boundary.

---

## What NOT to Do in Production Axum

```rust
// NEVER in a handler:
let user = db.find(id).unwrap();         // panics = one request kills all others
let user = db.find(id).expect("...");    // same — panic in async context

// AVOID in handlers:
fn get_user() -> Result<Json<User>, Box<dyn Error>>  // type erased — hard to match

// AVOID returning raw strings as errors:
fn get_user() -> Result<Json<User>, String>  // no structure, no HTTP status control

// CORRECT:
async fn get_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<User>, AppError> {
    let user = state.db.find(id).await?.ok_or(AppError::NotFound)?;
    Ok(Json(user))
}
```
