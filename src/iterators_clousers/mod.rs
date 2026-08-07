#![allow(dead_code)]

pub mod run_mod;

// =====================================================================
// JUNIOR
// =====================================================================

// 1. Closure basics
pub fn demo_closure_basics() {
    let add = |x: i32, y: i32| x + y;
    let square = |x: i32| x * x;
    let greet = |name: &str| format!("hello {name}");
    let print_num = |x: i32| println!("{x}");

    println!("[closure] add:    {}", add(2, 3));
    println!("[closure] square: {}", square(5));
    println!("[closure] greet:  {}", greet("Mishal"));
    print_num(42);
}

// =====================================================================

// 2. Capturing variables
pub fn demo_capturing() {
    let base = 10;

    // borrow — base is still usable after
    let add_base = |x| x + base;
    println!("[capture] borrow:     {} + {} = {}", 5, base, add_base(5));

    // move — ownership moved into closure
    let name = String::from("Mishal");
    let greet = move || format!("hello {name}");
    // println!("{name}"); // ERROR — name moved into closure
    println!("[capture] move:       {}", greet());

    // mut closure — captures mutably
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("[capture] mut count:  {}", increment());
    println!("[capture] mut count:  {}", increment());
}

// =====================================================================

// 3. iter() vs iter_mut() vs into_iter()
pub fn demo_iter_types() {
    let nums = vec![1, 2, 3];

    // iter() — borrows — gives &T
    for n in nums.iter() {
        print!("{n} "); // n is &i32
    }
    println!("← iter()");

    // iter_mut() — mutably borrows — gives &mut T
    let mut nums2 = vec![1, 2, 3];
    for n in nums2.iter_mut() {
        *n *= 10;
    }
    println!("[iter_mut] {:?}", nums2); // [10, 20, 30]

    // into_iter() — takes ownership — gives T
    let nums3 = vec![1, 2, 3];
    let doubled: Vec<i32> = nums3.into_iter().map(|n| n * 2).collect();
    println!("[into_iter] {:?}", doubled); // [2, 4, 6]
                                           // nums3 is moved — cannot use it anymore
}

// =====================================================================

// 4. .map() — transform each element
pub fn demo_map() {
    let nums = vec![1, 2, 3, 4, 5];

    let squared: Vec<i32> = nums.iter().map(|n| n * n).collect();
    println!("[map] squared:    {:?}", squared);

    let names = vec!["mishal", "ali", "ahmed"];
    let upper: Vec<String> = names.iter().map(|n| n.to_uppercase()).collect();
    println!("[map] uppercase:  {:?}", upper);
}

// =====================================================================

// 5. .filter() — keep matching elements
pub fn demo_filter() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let evens: Vec<&i32> = nums.iter().filter(|n| *n % 2 == 0).collect();
    println!("[filter] evens:    {:?}", evens);

    let words = vec!["rust", "go", "python", "ruby", "zig"];
    let short: Vec<&&str> = words.iter().filter(|w| w.len() <= 3).collect();
    println!("[filter] short:    {:?}", short);
}

// =====================================================================

// 6. .collect() — consume iterator into a collection
pub fn demo_collect() {
    let nums = vec![1, 2, 3, 4, 5];

    // collect into Vec
    let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
    println!("[collect] Vec:     {:?}", doubled);

    // collect into String
    let chars = vec!['R', 'u', 's', 't'];
    let word: String = chars.into_iter().collect();
    println!("[collect] String:  {}", word);

    // collect into HashMap
    use std::collections::HashMap;
    let pairs = vec![("name", "Mishal"), ("lang", "Rust")];
    let map: HashMap<&str, &str> = pairs.into_iter().collect();
    println!("[collect] HashMap: {:?}", map);
}

// =====================================================================

// 7. .for_each() — run a closure, return nothing
pub fn demo_for_each() {
    let nums = vec![1, 2, 3, 4, 5];

    nums.iter().for_each(|n| print!("{n} "));
    println!("← for_each");

    // real use — logging, sending events
    let errors = vec!["db timeout", "auth failed"];
    errors
        .iter()
        .for_each(|e| println!("[for_each] error: {e}"));
}

// =====================================================================

// 8. .count() .sum() .max() .min()
pub fn demo_aggregators() {
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];

    println!("[agg] count: {}", nums.iter().count());
    println!("[agg] sum:   {}", nums.iter().sum::<i32>());
    println!("[agg] max:   {:?}", nums.iter().max());
    println!("[agg] min:   {:?}", nums.iter().min());
}

// =====================================================================
// INTERMEDIATE
// =====================================================================

// 9. .chain() — join two iterators
pub fn demo_chain() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];

    let combined: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
    println!("[chain] {:?}", combined); // [1,2,3,4,5,6]
}

// =====================================================================

// 10. .enumerate() — pair each element with its index
pub fn demo_enumerate() {
    let names = vec!["Mishal", "Ali", "Ahmed"];

    for (i, name) in names.iter().enumerate() {
        println!("[enumerate] {i}: {name}");
    }
}

// =====================================================================

// 11. .zip() — pair elements from two iterators
pub fn demo_zip() {
    let keys = vec!["name", "lang", "level"];
    let values = vec!["Mishal", "Rust", "junior"];

    let pairs: Vec<(&&str, &&str)> = keys.iter().zip(values.iter()).collect();
    for (k, v) in &pairs {
        println!("[zip] {k}: {v}");
    }
}

// =====================================================================

// 12. .flat_map() — map then flatten
pub fn demo_flat_map() {
    let sentences = vec!["hello world", "rust is fast"];

    let words: Vec<&str> = sentences
        .iter()
        .flat_map(|s| s.split_whitespace())
        .collect();

    println!("[flat_map] {:?}", words);
    // ["hello", "world", "rust", "is", "fast"]
}

// =====================================================================

// 13. .take() and .skip()
pub fn demo_take_skip() {
    let nums: Vec<i32> = (1..=10).collect();

    let first3: Vec<&i32> = nums.iter().take(3).collect();
    println!("[take] first 3:  {:?}", first3);

    let skip3: Vec<&i32> = nums.iter().skip(3).collect();
    println!("[skip] after 3:  {:?}", skip3);

    // pagination pattern
    let page = 2;
    let page_size = 3;
    let page_data: Vec<&i32> = nums
        .iter()
        .skip((page - 1) * page_size)
        .take(page_size)
        .collect();
    println!("[take+skip] page {page}: {:?}", page_data);
}

// =====================================================================

// 14. .find() and .position()
pub fn demo_find_position() {
    let nums = vec![10, 20, 30, 40, 50];

    let found = nums.iter().find(|&&n| n > 25);
    println!("[find]     {:?}", found); // Some(30)

    let pos = nums.iter().position(|&n| n == 30);
    println!("[position] {:?}", pos); // Some(2)
}

// =====================================================================

// 15. .any() and .all()
pub fn demo_any_all() {
    let nums = vec![2, 4, 6, 8];

    println!("[any] has odd:      {}", nums.iter().any(|n| n % 2 != 0)); // false
    println!("[all] all even:     {}", nums.iter().all(|n| n % 2 == 0)); // true

    // real use — permission check
    let roles = vec!["viewer", "editor"];
    let can_edit = roles.iter().any(|r| *r == "editor" || *r == "admin");
    println!("[any] can edit:     {}", can_edit);
}

// =====================================================================

// 16. .fold() — accumulate across all elements
pub fn demo_fold() {
    let nums = vec![1, 2, 3, 4, 5];

    // sum using fold
    let sum = nums.iter().fold(0, |acc, n| acc + n);
    println!("[fold] sum:     {}", sum);

    // build a string
    let words = vec!["Rust", "is", "awesome"];
    let sentence = words.iter().fold(String::new(), |mut acc, w| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(w);
        acc
    });
    println!("[fold] sentence: {}", sentence);
}

// =====================================================================

// 17. Fn, FnMut, FnOnce
pub fn demo_fn_traits() {
    // Fn — borrows immutably, can call multiple times
    fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(x)
    }
    println!("[Fn]     {}", apply(|x| x * 2, 5));

    // FnMut — borrows mutably, can call multiple times
    fn apply_mut<F: FnMut() -> i32>(mut f: F) -> i32 {
        f()
    }
    let mut count = 0;
    println!(
        "[FnMut]  {}",
        apply_mut(|| {
            count += 1;
            count
        })
    );

    // FnOnce — takes ownership, can call only once
    fn apply_once<F: FnOnce() -> String>(f: F) -> String {
        f()
    }
    let name = String::from("Mishal");
    println!("[FnOnce] {}", apply_once(move || format!("hello {name}")));
}

// =====================================================================

// 18. Returning closures from functions
pub fn demo_returning_closures() {
    // impl Fn — when return type is known at compile time
    fn multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }

    let double = multiplier(2);
    let triple = multiplier(3);
    println!("[return closure] double 5: {}", double(5));
    println!("[return closure] triple 5: {}", triple(5));

    // Box<dyn Fn> — when returning different closures at runtime
    fn get_op(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
        match op {
            "add" => Box::new(|a, b| a + b),
            "mul" => Box::new(|a, b| a * b),
            _ => Box::new(|a, _| a),
        }
    }

    let op = get_op("add");
    println!("[Box<dyn Fn>] add: {}", op(3, 4));
}

// =====================================================================
// ADVANCED
// =====================================================================

// 19. Custom Iterator
pub struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    pub fn new(max: u32) -> Self {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

pub fn demo_custom_iterator() {
    let counter = Counter::new(5);
    let nums: Vec<u32> = counter.collect();
    println!("[custom iter] {:?}", nums);

    // works with all iterator methods
    let sum: u32 = Counter::new(5).filter(|n| n % 2 != 0).sum();
    println!("[custom iter] odd sum: {}", sum);
}

// =====================================================================

// 20. .peekable()
pub fn demo_peekable() {
    let nums = vec![1, 2, 3, 4, 5];
    let mut iter = nums.iter().peekable();

    while let Some(&val) = iter.peek() {
        if val % 2 == 0 {
            iter.next(); // consume even
        } else {
            println!("[peekable] odd: {}", iter.next().unwrap());
        }
    }
}

// =====================================================================

// 21. .windows() and .chunks()
pub fn demo_windows_chunks() {
    let nums = vec![1, 2, 3, 4, 5];

    // windows — overlapping slices
    print!("[windows] ");
    for w in nums.windows(3) {
        print!("{:?} ", w);
    }
    println!();

    // chunks — non-overlapping slices
    print!("[chunks]  ");
    for c in nums.chunks(2) {
        print!("{:?} ", c);
    }
    println!();
}

// =====================================================================

// 22. Iterator adapters as function arguments
pub fn sum_of_squares(iter: impl Iterator<Item = i32>) -> i32 {
    iter.map(|x| x * x).sum()
}

pub fn demo_iter_as_arg() {
    let nums = vec![1, 2, 3, 4, 5];
    let result = sum_of_squares(nums.into_iter());
    println!("[iter arg] sum of squares: {}", result);

    let result2 = sum_of_squares(1..=5);
    println!("[iter arg] range result:   {}", result2);
}

// =====================================================================

// 23. move closures — required for threads and async
pub fn demo_move_closures() {
    use std::thread;

    let message = String::from("hello from thread");

    // move — ownership of message moves into the closure
    let handle = thread::spawn(move || {
        println!("[move] {message}");
    });

    handle.join().unwrap();
    // println!("{message}"); // ERROR — message was moved
}

// =====================================================================

// 24. Closures in structs
pub struct Pipeline<F>
where
    F: Fn(i32) -> i32,
{
    transform: F,
}

impl<F: Fn(i32) -> i32> Pipeline<F> {
    pub fn new(transform: F) -> Self {
        Pipeline { transform }
    }

    pub fn run(&self, data: Vec<i32>) -> Vec<i32> {
        data.into_iter().map(|x| (self.transform)(x)).collect()
    }
}

pub fn demo_closure_in_struct() {
    let double_pipeline = Pipeline::new(|x| x * 2);
    let result = double_pipeline.run(vec![1, 2, 3, 4, 5]);
    println!("[struct closure] doubled: {:?}", result);

    let square_pipeline = Pipeline::new(|x| x * x);
    let result2 = square_pipeline.run(vec![1, 2, 3, 4, 5]);
    println!("[struct closure] squared: {:?}", result2);
}
