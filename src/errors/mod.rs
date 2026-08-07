#![allow(dead_code)]
use std::fmt;
use std::num::ParseIntError;

// =====================================================================
// 1. PANIC — unrecoverable error
// =====================================================================

#[allow(dead_code)]
pub fn get_element(v: &Vec<i32>, index: usize) -> i32 {
    if index >= v.len() {
        panic!("index out of bounds — this is a bug, not a user error");
    }
    v[index]
}

// =====================================================================
// 2. OPTION<T> — value may or may not exist
// =====================================================================

#[allow(dead_code)]
pub fn is_sqrt(x: f64) -> Option<f64> {
    if x >= 0.0 {
        Some(x.sqrt())
    } else {
        None
    }
}

// =====================================================================
// 3. RESULT<T, E> — success or recoverable failure
// =====================================================================

#[allow(dead_code)]
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("denominator cannot be zero"))
    } else {
        Ok(a / b)
    }
}

// =====================================================================
// 4. UNWRAP — extract value or panic
// =====================================================================

#[allow(dead_code)]
pub fn demo_unwrap() {
    let good: Result<i32, String> = Ok(10);
    let val = good.unwrap();
    println!("[unwrap] Ok value: {}", val);

    let present: Option<i32> = Some(42);
    let val = present.unwrap();
    println!("[unwrap] Some value: {}", val);

    // Uncomment to see panic:
    // let bad: Result<i32, String> = Err(String::from("fail"));
    // bad.unwrap(); // panics here
}

// =====================================================================
// 5. EXPECT — extract value or panic with a message
// =====================================================================

pub fn demo_expect() {
    let result: Result<i32, String> = Ok(99);
    let val = result.expect("this should always succeed");
    println!("[expect] value: {}", val);

    // Uncomment to see a meaningful panic message:
    // let bad: Result<i32, String> = Err(String::from("oops"));
    // bad.expect("configuration value must be present at startup");
}

// =====================================================================
// 6. MATCH — explicitly handle every variant
// =====================================================================

pub fn demo_match() {
    // match on Result
    match divide(10.0, 2.0) {
        Ok(val) => println!("[match] result: {}", val),
        Err(e) => println!("[match] error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(val) => println!("[match] result: {}", val),
        Err(e) => println!("[match] error: {}", e),
    }

    // match on Option
    match is_sqrt(9.0) {
        Some(val) => println!("[match] sqrt: {}", val),
        None => println!("[match] no sqrt for negative numbers"),
    }

    match is_sqrt(-4.0) {
        Some(val) => println!("[match] sqrt: {}", val),
        None => println!("[match] no sqrt for negative numbers"),
    }
}

// =====================================================================
// 7. ? OPERATOR — propagate errors early
// =====================================================================

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| e.to_string())
}

fn double_number(s: &str) -> Result<i32, String> {
    let n = parse_number(s)?; // returns early if Err
    Ok(n * 2)
}

pub fn demo_question_mark() {
    match double_number("21") {
        Ok(val) => println!("[?] doubled: {}", val),
        Err(e) => println!("[?] error: {}", e),
    }

    match double_number("abc") {
        Ok(val) => println!("[?] doubled: {}", val),
        Err(e) => println!("[?] error: {}", e),
    }
}

// =====================================================================
// 8. CUSTOM ERROR TYPES — model your domain failures
// =====================================================================

#[derive(Debug)]
pub enum RegistrationError {
    EmptyName,
    AgeTooLow(u8),
    EmailAlreadyExists(String),
}

pub fn register(name: &str, age: u8, email_exists: bool) -> Result<(), RegistrationError> {
    if name.trim().is_empty() {
        return Err(RegistrationError::EmptyName);
    }
    if age < 18 {
        return Err(RegistrationError::AgeTooLow(age));
    }
    if email_exists {
        return Err(RegistrationError::EmailAlreadyExists(String::from(
            "user@example.com",
        )));
    }
    Ok(())
}

pub fn demo_custom_error() {
    match register("Mishal", 20, false) {
        Ok(()) => println!("[custom error] registered successfully"),
        Err(e) => println!("[custom error] failed: {:?}", e),
    }

    match register("", 20, false) {
        Ok(()) => println!("[custom error] registered successfully"),
        Err(e) => println!("[custom error] failed: {:?}", e),
    }

    match register("Ali", 15, false) {
        Ok(()) => println!("[custom error] registered successfully"),
        Err(e) => println!("[custom error] failed: {:?}", e),
    }
}

// =====================================================================
// 9. DISPLAY + ERROR TRAIT — readable messages
// =====================================================================

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistrationError::EmptyName => write!(f, "name cannot be empty"),
            RegistrationError::AgeTooLow(age) => {
                write!(f, "age {} is below the minimum of 18", age)
            }
            RegistrationError::EmailAlreadyExists(email) => {
                write!(f, "email {} is already registered", email)
            }
        }
    }
}

impl std::error::Error for RegistrationError {}

pub fn demo_display_error() {
    let err = RegistrationError::AgeTooLow(15);
    println!("[Display] Debug  : {:?}", err);
    println!("[Display] Display: {}", err); // uses our Display impl
}

// =====================================================================
// 10. FROM TRAIT — automatic error conversion
// =====================================================================

#[derive(Debug)]
pub enum AppError {
    Parse(ParseIntError),
    Logic(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "parse error: {}", e),
            AppError::Logic(msg) => write!(f, "logic error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn parse_port(s: &str) -> Result<u16, AppError> {
    let port = s.parse::<u16>()?; // ParseIntError auto-converts to AppError via From
    if port < 1024 {
        return Err(AppError::Logic(String::from("port must be >= 1024")));
    }
    Ok(port)
}

pub fn demo_from() {
    match parse_port("8080") {
        Ok(p) => println!("[From] port: {}", p),
        Err(e) => println!("[From] error: {}", e),
    }

    match parse_port("abc") {
        Ok(p) => println!("[From] port: {}", p),
        Err(e) => println!("[From] error: {}", e),
    }

    match parse_port("80") {
        Ok(p) => println!("[From] port: {}", p),
        Err(e) => println!("[From] error: {}", e),
    }
}

// =====================================================================
// 11. BOX<DYN ERROR> — multiple error types behind one interface
// =====================================================================

fn read_and_double(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let n = s.parse::<i32>()?; // ParseIntError  \
    Ok(n * 2) //                 > both auto-boxed
}

pub fn demo_box_error() {
    match read_and_double("21") {
        Ok(val) => println!("[Box<dyn Error>] result: {}", val),
        Err(e) => println!("[Box<dyn Error>] error: {}", e),
    }

    match read_and_double("xyz") {
        Ok(val) => println!("[Box<dyn Error>] result: {}", val),
        Err(e) => println!("[Box<dyn Error>] error: {}", e),
    }
}

// =====================================================================
// 12. THISERROR — typed errors with less boilerplate
// =====================================================================

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("invalid port number")]
    InvalidPort(#[from] ParseIntError),

    #[error("port {0} is reserved — use a port above 1024")]
    ReservedPort(u16),
}

fn load_port(s: &str) -> Result<u16, ConfigError> {
    let port = s.parse::<u16>()?;
    if port < 1024 {
        return Err(ConfigError::ReservedPort(port));
    }
    Ok(port)
}

pub fn demo_thiserror() {
    match load_port("8080") {
        Ok(p) => println!("[thiserror] port: {}", p),
        Err(e) => println!("[thiserror] error: {}", e),
    }

    match load_port("abc") {
        Ok(p) => println!("[thiserror] port: {}", p),
        Err(e) => println!("[thiserror] error: {}", e),
    }

    match load_port("80") {
        Ok(p) => println!("[thiserror] port: {}", p),
        Err(e) => println!("[thiserror] error: {}", e),
    }
}

// =====================================================================
// 13. ANYHOW — flexible errors with context
// =====================================================================

fn parse_with_context(s: &str) -> anyhow::Result<u16> {
    let port = s
        .parse::<u16>()
        .map_err(|_| anyhow::anyhow!("'{}' is not a valid port number", s))?;

    anyhow::ensure!(port >= 1024, "port {} is reserved, must be >= 1024", port);

    Ok(port)
}

pub fn demo_anyhow() {
    match parse_with_context("8080") {
        Ok(p) => println!("[anyhow] port: {}", p),
        Err(e) => println!("[anyhow] error: {}", e),
    }

    match parse_with_context("abc") {
        Ok(p) => println!("[anyhow] port: {}", p),
        Err(e) => println!("[anyhow] error: {}", e),
    }

    match parse_with_context("80") {
        Ok(p) => println!("[anyhow] port: {}", p),
        Err(e) => println!("[anyhow] error: {}", e),
    }
}

// =====================================================================
// 14. AXUM ERRORS — IntoResponse pattern (no axum dep needed to learn)
//
// In a real Axum app the handler returns Result<T, AppHttpError>.
// AppHttpError implements IntoResponse so Axum can turn it into an
// HTTP response automatically.
//
// async fn get_user() -> Result<Json<User>, AppHttpError> {
//     let user = db.find(1).ok_or(AppHttpError::NotFound)?;
//     Ok(Json(user))
// }
//
// impl IntoResponse for AppHttpError {
//     fn into_response(self) -> Response {
//         let (status, msg) = match self {
//             AppHttpError::NotFound     => (StatusCode::NOT_FOUND,            "not found"),
//             AppHttpError::BadRequest   => (StatusCode::BAD_REQUEST,          "bad request"),
//             AppHttpError::ServerError  => (StatusCode::INTERNAL_SERVER_ERROR, "server error"),
//         };
//         (status, Json(json!({ "error": msg }))).into_response()
//     }
// }
// =====================================================================
