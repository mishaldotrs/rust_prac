#[allow(dead_code)]
pub fn run() {

    // ============================================
    // OWNERSHIP RULES
    // ============================================
    // Rule 1: every value has an owner
    let a = 42;                           // a owns 42
    let s1 = String::from("Rust");        // s1 owns the String

    // Rule 2: only one owner at a time
    let b = a;                            // Copy → both a and b are valid
    let s2 = s1;                          // Move → only s2 is valid now

    println!("a = {}, b = {}", a, b);     // ✅ works
    // println!("{}", s1);                // ❌ Error
    println!("{}", s2);                   // ✅ works

    // Rule 3: value is dropped when owner leaves scope
    {
        let c = 100;
        let s3 = String::from("temp");
        println!("Inside scope: {} {}", c, s3);
    } // c and s3 are dropped here

    println!("Still alive: {} {} {}", a, b, s2);

    // ============================================
    // BORROWING & REFERENCES RULES
    // ============================================

    let mut s = String::from("hello");

    // ------------------------------------------
    // Rule 1: You can have either one mutable reference
    //         OR any number of immutable references
    //         (but not both at the same time)
    // ------------------------------------------

    // Immutable references (kitne bhi allowed)
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2); // ✅ works


    // Mutable reference (sirf ek allowed)
    let r3 = &mut s;
    
    r3.push_str(" world");
    println!("{}", r3); // ✅ works
    

    // ------------------------------------------
    // Rule 2: References must always be valid
    //         (Dangling references allowed nahi hain)
    // ------------------------------------------

    // Galat example (yeh compile nahi hoga):
    // let r;
    // {
    //     let temp = String::from("temp");
    //     r = &temp;           // temp yahan drop ho jayega
    // }                       // r ab dangling reference ban jayega
    // println!("{}", r);      // ❌ Error

    // Sahi tareeka:
    let r4 = &s;               // s abhi bhi zinda hai
    println!("{}", r4);        // ✅ works

    // ------------------------------------------
    // Rule 3: Borrow ends after its last use
    //         (Non-Lexical Lifetimes - NLL)
    // ------------------------------------------

    let r5 = &mut s;
    r5.push_str("!");
    println!("{}", r5);        // ← r5 ka last use

    // Ab r5 ka borrow khatam ho chuka hai
    println!("{}", s);         // ✅ ab original ko use kar sakte ho
}