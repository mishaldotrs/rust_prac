#![allow(dead_code)]

#[path = "ownership_borrowing/mod.rs"]
mod ownership_borrowing;

#[path = "traits_generics/mod.rs"]
mod traits_generics;

#[path = "errors/mod.rs"]
mod errors;

#[path = "enums-pattern_matching/mod.rs"]
mod enums_pattern_matching;

#[path = "iterators_clousers/mod.rs"]
mod iterators_clousers;

fn main() {
    ownership_borrowing::run_mod::run();
    traits_generics::run_mod::run();
    errors::run_mod::run();
    enums_pattern_matching::run_mod::run();
    iterators_clousers::run_mod::run();
}
