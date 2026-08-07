use super::*;

pub fn run() {
    println!("\n===== 1.  CLOSURE BASICS =====");
    demo_closure_basics();

    println!("\n===== 2.  CAPTURING =====");
    demo_capturing();

    println!("\n===== 3.  ITER TYPES =====");
    demo_iter_types();

    println!("\n===== 4.  MAP =====");
    demo_map();

    println!("\n===== 5.  FILTER =====");
    demo_filter();

    println!("\n===== 6.  COLLECT =====");
    demo_collect();

    println!("\n===== 7.  FOR EACH =====");
    demo_for_each();

    println!("\n===== 8.  AGGREGATORS =====");
    demo_aggregators();

    println!("\n===== 9.  CHAIN =====");
    demo_chain();

    println!("\n===== 10. ENUMERATE =====");
    demo_enumerate();

    println!("\n===== 11. ZIP =====");
    demo_zip();

    println!("\n===== 12. FLAT MAP =====");
    demo_flat_map();

    println!("\n===== 13. TAKE & SKIP =====");
    demo_take_skip();

    println!("\n===== 14. FIND & POSITION =====");
    demo_find_position();

    println!("\n===== 15. ANY & ALL =====");
    demo_any_all();

    println!("\n===== 16. FOLD =====");
    demo_fold();

    println!("\n===== 17. FN TRAITS =====");
    demo_fn_traits();

    println!("\n===== 18. RETURNING CLOSURES =====");
    demo_returning_closures();

    println!("\n===== 19. CUSTOM ITERATOR =====");
    demo_custom_iterator();

    println!("\n===== 20. PEEKABLE =====");
    demo_peekable();

    println!("\n===== 21. WINDOWS & CHUNKS =====");
    demo_windows_chunks();

    println!("\n===== 22. ITER AS ARG =====");
    demo_iter_as_arg();

    println!("\n===== 23. MOVE CLOSURES =====");
    demo_move_closures();

    println!("\n===== 24. CLOSURE IN STRUCT =====");
    demo_closure_in_struct();
}
