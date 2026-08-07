// =====================================================================
// 1. IF LET
// =====================================================================

pub fn demo_if_let() {
    let user: Option<String> = Some(String::from("Mishal"));

    if let Some(name) = user {
        println!("[if let] logged in as {name}");
    } else {
        println!("[if let] no user found");
    }

    let port: Option<u16> = None;

    if let Some(p) = port {
        println!("[if let] port: {p}");
    } else {
        println!("[if let] using default port 8080");
    }
}

// =====================================================================
// 2. DESTRUCTURING
// =====================================================================

#[allow(dead_code)]
#[derive(Debug)]
pub enum Event {
    UserCreated { id: u32, name: String },
    UserDeleted(u32),
    UserUpdated { id: u32, new_name: String },
}

pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn demo_destructuring() {
    // enum destructuring
    let events = vec![
        Event::UserCreated {
            id: 1,
            name: String::from("Mishal"),
        },
        Event::UserDeleted(2),
        Event::UserUpdated {
            id: 3,
            new_name: String::from("Ali"),
        },
    ];

    for event in events {
        match event {
            Event::UserCreated { id, name } => {
                println!("[destructure] created  → id:{id} name:{name}")
            }
            Event::UserDeleted(id) => println!("[destructure] deleted  → id:{id}"),
            Event::UserUpdated { id, new_name } => {
                println!("[destructure] updated  → id:{id} name:{new_name}")
            }
        }
    }

    // struct destructuring
    let config = Config {
        host: String::from("localhost"),
        port: 8080,
    };
    let Config { host, port } = config;
    println!("[destructure] connecting to {host}:{port}");

    // tuple destructuring
    let (status, message) = (200_u16, "ok");
    println!("[destructure] {status}: {message}");
}

// =====================================================================
// 3. STATE MACHINE
// =====================================================================

#[allow(dead_code)]
#[derive(Debug)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipped { tracking_id: String },
    Delivered,
    Cancelled(String),
}

impl OrderStatus {
    pub fn confirm(self) -> Result<Self, String> {
        match self {
            OrderStatus::Pending => Ok(OrderStatus::Confirmed),
            other => Err(format!("{other:?} cannot be confirmed")),
        }
    }

    pub fn ship(self, tracking_id: String) -> Result<Self, String> {
        match self {
            OrderStatus::Confirmed => Ok(OrderStatus::Shipped { tracking_id }),
            other => Err(format!("{other:?} cannot be shipped")),
        }
    }

    pub fn cancel(self, reason: String) -> Result<Self, String> {
        match self {
            OrderStatus::Delivered => Err(String::from("delivered orders cannot be cancelled")),
            _ => Ok(OrderStatus::Cancelled(reason)),
        }
    }
}

pub fn demo_state_machine() {
    // valid transition
    let order = OrderStatus::Pending;
    let order = order.confirm().unwrap();
    let order = order.ship(String::from("TRK-001")).unwrap();
    println!("[state machine] valid flow  → {:?}", order);

    // invalid transition
    let order2 = OrderStatus::Pending;
    match order2.ship(String::from("TRK-002")) {
        Ok(s) => println!("[state machine] shipped → {:?}", s),
        Err(e) => println!("[state machine] blocked → {e}"),
    }

    // cancel
    let order3 = OrderStatus::Confirmed;
    let order3 = order3.cancel(String::from("customer request")).unwrap();
    println!("[state machine] cancelled  → {:?}", order3);

    // cannot cancel delivered
    let order4 = OrderStatus::Delivered;
    match order4.cancel(String::from("too late")) {
        Ok(s) => println!("[state machine] cancelled → {:?}", s),
        Err(e) => println!("[state machine] blocked   → {e}"),
    }
}

// =====================================================================
// 4. RESULT HANDLING
// =====================================================================

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    NotFound,
    Unauthorized,
    BadRequest(String),
    Database(String),
}

pub fn get_user(id: u32, is_admin: bool) -> Result<String, AppError> {
    if !is_admin {
        return Err(AppError::Unauthorized);
    }

    if id == 0 {
        return Err(AppError::BadRequest(String::from("id cannot be zero")));
    }

    match id {
        1 => Ok(String::from("Mishal")),
        2 => Ok(String::from("Ali")),
        _ => Err(AppError::NotFound),
    }
}

pub fn demo_result_handling() {
    let test_cases = vec![
        (1, true),  // ok
        (99, true), // not found
        (1, false), // unauthorized
        (0, true),  // bad request
    ];

    for (id, is_admin) in test_cases {
        match get_user(id, is_admin) {
            Ok(name) => println!("[result] 200 → user: {name}"),
            Err(AppError::NotFound) => println!("[result] 404 → not found"),
            Err(AppError::Unauthorized) => println!("[result] 401 → unauthorized"),
            Err(AppError::BadRequest(msg)) => println!("[result] 400 → {msg}"),
            Err(AppError::Database(msg)) => println!("[result] 500 → {msg}"),
        }
    }
}

// =====================================================================
// 5. API DESIGN — all 4 combined
// (Axum IntoResponse shown as a pattern without the crate)
// =====================================================================

pub struct User {
    pub id: u32,
    pub name: String,
    pub ban_reason: Option<String>,
}

fn find_user(id: u32) -> Option<User> {
    match id {
        1 => Some(User {
            id: 1,
            name: String::from("Mishal"),
            ban_reason: None,
        }),
        2 => Some(User {
            id: 2,
            name: String::from("Ali"),
            ban_reason: Some(String::from("spam")),
        }),
        _ => None,
    }
}

pub fn demo_api_design() {
    for id in [1, 2, 3] {
        let result = handle_get_user(id);
        match result {
            Ok(msg) => println!("[api] 200 → {msg}"),
            Err(AppError::NotFound) => println!("[api] 404 → not found"),
            Err(AppError::Unauthorized) => println!("[api] 401 → banned"),
            Err(AppError::BadRequest(msg)) => println!("[api] 400 → {msg}"),
            Err(AppError::Database(msg)) => println!("[api] 500 → {msg}"),
        }
    }
}

fn handle_get_user(id: u32) -> Result<String, AppError> {
    // if let — check optional DB result
    if let Some(user) = find_user(id) {
        // destructuring — unpack struct fields
        let User {
            id,
            name,
            ban_reason,
        } = user;

        // if let — check optional field
        if let Some(_reason) = ban_reason {
            return Err(AppError::Unauthorized);
        }

        Ok(format!("id:{id} name:{name}"))
    } else {
        Err(AppError::NotFound)
    }
}
