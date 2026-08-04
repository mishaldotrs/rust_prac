use std::collections::HashMap;

trait UrlStore {
    fn save(&mut self, short: String, long: String);
    fn get(&self, short: &str) -> Option<String>;
}

struct MemoryStore {
    data: HashMap<String, String>,
}

impl MemoryStore {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl UrlStore for MemoryStore {
    fn save(&mut self, short: String, long: String) {
        self.data.insert(short, long);
    }

    fn get(&self, short: &str) -> Option<String> {
        self.data.get(short).cloned()
    }
}

struct PostgresStore {}

impl UrlStore for PostgresStore {
    fn save(&mut self, short: String, long: String) {
        println!("Saving to Postgres: {} → {}", short, long);
    }

    fn get(&self, short: &str) -> Option<String> {
        println!("Getting from Postgres: {}", short);
        None
    }
}

fn create_short_url<S: UrlStore>(store: &mut S, long_url: &str) -> String {
    let short = generate_short_code();
    store.save(short.clone(), long_url.to_string());
    short
}

fn get_long_url<S: UrlStore>(store: &S, short: &str) -> Option<String> {
    store.get(short)
}

fn generate_short_code() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{:x}", time).chars().rev().take(6).collect()
}

fn run() {
    let mut memory_store = MemoryStore::new();
    let short1 = create_short_url(&mut memory_store, "https://example.com/very/long/url");
    println!("Memory Short URL: {}", short1);

    match get_long_url(&memory_store, &short1) {
        Some(long) => println!("Memory Original URL: {}", long),
        None => println!("Not found"),
    }

    println!("-----------------------");

    let mut postgres_store = PostgresStore {};
    let short2 = create_short_url(&mut postgres_store, "https://example.com/another/long/url");
    println!("Postgres Short URL: {}", short2);

    match get_long_url(&postgres_store, &short2) {
        Some(long) => println!("Postgres Original URL: {}", long),
        None => println!("Not found"),
    }
}