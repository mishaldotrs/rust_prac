#![allow(dead_code)]

#[path = "iterators_clousers/mod.rs"]
mod iterators_clousers;

fn main() {
   
    // ===== ITERATORS & CLOSURES =====
    println!("\n===== ITER 1.  CLOSURE BASICS =====");
    iterators_clousers::demo_closure_basics();

    println!("\n===== ITER 2.  CAPTURING =====");
    iterators_clousers::demo_capturing();

    println!("\n===== ITER 3.  ITER TYPES =====");
    iterators_clousers::demo_iter_types();

    println!("\n===== ITER 4.  MAP =====");
    iterators_clousers::demo_map();

    println!("\n===== ITER 5.  FILTER =====");
    iterators_clousers::demo_filter();

    println!("\n===== ITER 6.  COLLECT =====");
    iterators_clousers::demo_collect();

    println!("\n===== ITER 7.  FOR EACH =====");
    iterators_clousers::demo_for_each();

    println!("\n===== ITER 8.  AGGREGATORS =====");
    iterators_clousers::demo_aggregators();

    println!("\n===== ITER 9.  CHAIN =====");
    iterators_clousers::demo_chain();

    println!("\n===== ITER 10. ENUMERATE =====");
    iterators_clousers::demo_enumerate();

    println!("\n===== ITER 11. ZIP =====");
    iterators_clousers::demo_zip();

    println!("\n===== ITER 12. FLAT MAP =====");
    iterators_clousers::demo_flat_map();

    println!("\n===== ITER 13. TAKE & SKIP =====");
    iterators_clousers::demo_take_skip();

    println!("\n===== ITER 14. FIND & POSITION =====");
    iterators_clousers::demo_find_position();

    println!("\n===== ITER 15. ANY & ALL =====");
    iterators_clousers::demo_any_all();

    println!("\n===== ITER 16. FOLD =====");
    iterators_clousers::demo_fold();

    println!("\n===== ITER 17. FN TRAITS =====");
    iterators_clousers::demo_fn_traits();

    println!("\n===== ITER 18. RETURNING CLOSURES =====");
    iterators_clousers::demo_returning_closures();

    println!("\n===== ITER 19. CUSTOM ITERATOR =====");
    iterators_clousers::demo_custom_iterator();

    println!("\n===== ITER 20. PEEKABLE =====");
    iterators_clousers::demo_peekable();

    println!("\n===== ITER 21. WINDOWS & CHUNKS =====");
    iterators_clousers::demo_windows_chunks();

    println!("\n===== ITER 22. ITER AS ARG =====");
    iterators_clousers::demo_iter_as_arg();

    println!("\n===== ITER 23. MOVE CLOSURES =====");
    iterators_clousers::demo_move_closures();

    println!("\n===== ITER 24. CLOSURE IN STRUCT =====");
    iterators_clousers::demo_closure_in_struct();
}
