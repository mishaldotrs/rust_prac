#![allow(dead_code)]

pub mod run_mod;

// =====================================================================
// 1. OWNERSHIP BASICS
// =====================================================================

pub fn demo_ownership() {
    let a = 42;
    let s1 = String::from("Rust");

    let b = a; // Copy — both valid
    let s2 = s1; // Move — only s2 valid

    println!("[ownership] a={} b={}", a, b);
    println!("[ownership] s2={}", s2);

    {
        let temp = String::from("temp");
        println!("[ownership] inside scope: {}", temp);
    } // temp dropped here

    println!("[ownership] still alive: {} {}", a, b);
}

// =====================================================================
// 2. MOVE SEMANTICS
// =====================================================================

pub fn demo_move() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved to s2
                 // println!("{}", s1);              // ERROR — s1 invalid
    println!("[move] s2: {}", s2);

    // move into function
    fn take_ownership(s: String) {
        println!("[move] inside fn: {}", s);
    } // s dropped here

    let s3 = String::from("world");
    take_ownership(s3);
    // println!("{}", s3);             // ERROR — s3 moved
}

// =====================================================================
// 3. COPY TYPES
// =====================================================================

pub fn demo_copy() {
    let x = 5;
    let y = x; // Copy — x still valid
    println!("[copy] x={} y={}", x, y);

    let a = true;
    let b = a;
    println!("[copy] a={} b={}", a, b);

    // Copy types: i32, f64, bool, char, tuples of Copy types
    let t1 = (1, 2);
    let t2 = t1;
    println!("[copy] t1={:?} t2={:?}", t1, t2);
}

// =====================================================================
// 4. CLONE — explicit deep copy
// =====================================================================

pub fn demo_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy — both valid
    println!("[clone] s1={} s2={}", s1, s2);

    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();
    println!("[clone] v1={:?} v2={:?}", v1, v2);
}

// =====================================================================
// 5. IMMUTABLE REFERENCES &T
// =====================================================================

pub fn demo_immutable_refs() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s; // multiple immutable refs — allowed
    println!("[&T] r1={} r2={}", r1, r2);
    println!("[&T] s still valid: {}", s);

    fn print_str(s: &String) {
        println!("[&T] borrowed: {}", s);
    }

    print_str(&s);
    println!("[&T] s after fn: {}", s); // s still valid
}

// =====================================================================
// 6. MUTABLE REFERENCES &mut T
// =====================================================================

pub fn demo_mutable_refs() {
    let mut s = String::from("hello");

    let r = &mut s;
    r.push_str(" world");
    println!("[&mut] {}", r);

    // only ONE mutable ref at a time
    // let r2 = &mut s;  // ERROR — two mut refs

    fn append(s: &mut String, text: &str) {
        s.push_str(text);
    }

    append(&mut s, "!");
    println!("[&mut] after fn: {}", s);
}

// =====================================================================
// 7. DANGLING REFERENCES — compiler prevents these
// =====================================================================

pub fn demo_dangling() {
    // This would be a dangling reference — compiler blocks it:
    //
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s   // ERROR — s dropped at end of fn, reference invalid
    // }
    //
    // Correct — return the owned value instead:

    fn no_dangle() -> String {
        let s = String::from("hello");
        s // ownership moved out — no dangling
    }

    let s = no_dangle();
    println!("[dangling] safe: {}", s);
}

// =====================================================================
// 8. NLL — Non-Lexical Lifetimes
// =====================================================================

pub fn demo_nll() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("[NLL] {} {}", r1, r2);
    // r1 and r2 last used here — borrow ends here

    let r3 = &mut s; // OK — immutable borrows already ended
    r3.push_str(" world");
    println!("[NLL] {}", r3);
}

// =====================================================================
// 9. SLICES
// =====================================================================

pub fn demo_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("[slice] '{}' '{}'", hello, world);

    let nums = vec![1, 2, 3, 4, 5];
    let middle = &nums[1..4];
    println!("[slice] middle: {:?}", middle);
}

// =====================================================================
// 10. OWNERSHIP IN FUNCTIONS
// =====================================================================

pub fn demo_ownership_in_fns() {
    fn gives_ownership() -> String {
        String::from("owned")
    }

    fn takes_and_gives_back(s: String) -> String {
        s // ownership returned to caller
    }

    let s1 = gives_ownership();
    println!("[fn ownership] s1: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2);  // ERROR — s2 moved
    println!("[fn ownership] s3: {}", s3);
}

// =====================================================================
// 11. BORROWING IN FUNCTIONS
// =====================================================================

pub fn demo_borrowing_in_fns() {
    fn length(s: &String) -> usize {
        s.len()
    }

    let s = String::from("hello");
    let len = length(&s);
    println!("[fn borrow] '{}' has {} chars", s, len);
    // s still valid — we only borrowed it
}

// =====================================================================
// 12. ARC — multiple ownership across threads
// =====================================================================

pub fn demo_arc() {
    use std::sync::Arc;

    let data = Arc::new(vec![1, 2, 3]);
    let data2 = Arc::clone(&data);

    let handle = std::thread::spawn(move || {
        println!("[Arc] in thread: {:?}", data2);
    });

    handle.join().unwrap();
    println!("[Arc] in main:   {:?}", data);
}
