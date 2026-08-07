#![allow(dead_code)]

#[path = "ownership_borrowing/mod.rs"]
mod ownership_borrowing;

#[path = "iterators_clousers/mod.rs"]
mod iterators_clousers;

fn main() {
    ownership_borrowing::run_mod::run();
    iterators_clousers::run_mod::run();
}
