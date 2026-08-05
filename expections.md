-> Ownership -------
                  1| -> Each value in Rust has an owner.
                  2| -> There can only be one owner at a time.
                  3| -> When the owner goes out of scope, the value will be dropped.

------------------------------------------------------------------------------------------------------------------------------------------------------------------------- 
-> Borrowing & Reference -------
                  1| -> At any given time, you can have either one mutable reference or any number of immutable references. ----------> Gives data races
                  2| -> References must always be valid. ----------> Gives dangling references -----> We have lifetimes
                              
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------            
            
// interiar mutablity == borrowing(Execptions in rust) -> ref cell mutex
// unsafe rust == row pointer (ownership & borrowing)
// rc arc == (single ownership rule expection)
// Cow = (borrowing & reference)
// Pin = (pointer & references)
// lifetimes = (3 rules)
// dangling references =  due to borrowing rule no - 2
// data races  = due to borrowing rule no - 1
// send and sync Trait = 
// lifetime rule with longest string (&str)
// 
// 


use std::cell::RefCell;


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }else if x.len() == y.len(){
        "both with same length"
    }else {
        y
    }
}

fn count_even_range(start: i32, end: i32) -> i32 {
    let mut count = 0;
    for i in start..=end {
        if i % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn main(){

    let result = count_even_range(1, 10);
    
    println!("count of even numbers: {}", result);

    let data = RefCell::new(5);

    {
        let mut mut_ref = data.borrow_mut();
        *mut_ref += 10;
    }

    println!("updated values:{}",data.borrow());

    let string1 = "abc";

    let string2 = "xyz";

    let result = longest(string1, string2);

    println!("the longest string is '{}'",result);
}


error in rust - 


fn divide(x: f64, y: f64) -> Result<f64, String>{
     if y == 0.0 {
        Err("denominator can not be zero".to_string())
     }else {
         Ok(x/y)
     }
}

fn main(){
  
    match divide(4.0, 2.0) {
            Ok(val) => println!("Result: {}", val),
            Err(e) => println!("Error: {}", e),
        }
}


---> traits & generics 

1. Traits
         → a contract that defines what a type CAN DO
         → only function signatures, no body
         → syntax:  trait Name { fn method(&self) -> ReturnType; }
         → Java's interface === Rust's trait

2. Trait impl
         → fulfilling the trait contract for a specific struct/enum
         → every function declared in the trait MUST be implemented
         → syntax:  impl TraitName for StructName { fn method(&self) -> ReturnType { ... } }

3. Default impl
         → write a default body for a function inside the trait itself
         → overriding it is OPTIONAL
         → types that don't override it automatically get the default
         → syntax:  trait Name { fn method(&self) -> String { String::from("default") } }

4. Generic
         → a placeholder type <T> that the caller decides
         → write one piece of code that works with any type
         → T, U, K, V — just conventions, you can use any name
         → syntax:  fn foo<T>(a: T) -> T { ... }
         → Option<T>, Result<T, E> — these are all generic

5. Trait bounds
         → restrict the generic T — only types that can do something
         → T: SomeTrait means T must implement SomeTrait
         → multiple bounds:  T: Trait1 + Trait2
         → where clause (clean syntax):  where T: Trait1 + Trait2
         → syntax:  fn foo<T: Add<Output=T>>(a: T, b: T) -> T { a + b }

6. impl Trait
         → shorthand syntax for trait bounds — no need to write full generic
         → in parameters:  fn foo(a: &impl SomeTrait) { ... }
         → in return type:  fn foo() -> impl SomeTrait { ... }  (hides the exact type)
         → limitation: does not guarantee two parameters are the same type

7. dyn Trait
         → the actual type is decided at RUNTIME, not compile time
         → must wrap in Box<dyn Trait> because size is unknown at compile time
         → allows storing different types together in a Vec
         → impl Trait = compile time | dyn Trait = runtime
         → syntax:  fn foo() -> Box<dyn SomeTrait> { Box::new(Dog) }

8. Associated Types
         → a type defined inside a trait, set by whoever implements it
         → type Output; — name is declared in trait, actual type set in impl
         → Iterator's Item, Add's Output — these are all associated types
         → syntax:  trait Foo { type Output; fn bar(&self) -> Self::Output; }

9. Blanket impl
         → a single generic impl that applies to ALL qualifying types at once
         → if a type implements one trait, it automatically gets another
         → std library: impl<T: Display> ToString for T — that's why 42.to_string() works
         → syntax:  impl<T: Display> PrintMe for T { ... }


-------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---> Learning Roadmap (Axum / Tokio Backend)

  1. Traits & Generics        ✅ done
         → contract, impl, default, generic <T>, bounds, impl Trait, dyn Trait, associated types, blanket impl

  2. Iterators & Closures
         → Iterator trait, .map() .filter() .collect()
         → closures — |x| x + 1
         → lazy evaluation
         → used everywhere in real code

  3. Error Handling
         → Result<T, E> and Option<T>
         → ? operator
         → custom error types
         → thiserror / anyhow crates (used in Axum apps)

  4. Lifetimes
         → 'a syntax
         → why references need lifetimes
         → lifetime elision rules

  5. Smart Pointers
         → Box<T>     — heap allocation
         → Rc<T>      — multiple owners (single thread)
         → Arc<T>     — multiple owners (multi thread)
         → RefCell<T> — interior mutability (single thread)
         → Mutex<T>   — interior mutability (multi thread)
         → Pin<T>     — prevent moving in memory

  6. Concurrency
         → threads — std::thread::spawn
         → Send + Sync marker traits
         → Arc + Mutex pattern
         → channels — mpsc

  7. Async / Await
         → Future trait
         → async fn, .await
         → Pin + Unpin
         → tokio runtime

  8. Axum + Tokio (Backend)
         → Router, handlers
         → extractors — Json, Path, Query, State
         → middleware — tower::Service
         → error handling in Axum
         → database — sqlx
