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


---> generics order 
1. Traits          → contract kya hota hai, kaise likhte hain
2. Trait impl      → kisi struct pe implement karna
3. Default impl    → optional override
4. Generic         → <T> kya hai
5. Trait bounds    → <T: SomeTrait> → dono saath mein -> 
6. impl Trait      → shorthand
7. dyn Trait       → runtime polymorphism
8. Associated Types
9. Blanket impl
