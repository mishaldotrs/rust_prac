use super::*;

pub fn run() {
    println!("\n===== 1.  OWNERSHIP BASICS =====");
    demo_ownership();

    println!("\n===== 2.  MOVE SEMANTICS =====");
    demo_move();

    println!("\n===== 3.  COPY TYPES =====");
    demo_copy();

    println!("\n===== 4.  CLONE =====");
    demo_clone();

    println!("\n===== 5.  IMMUTABLE REFS &T =====");
    demo_immutable_refs();

    println!("\n===== 6.  MUTABLE REFS &mut T =====");
    demo_mutable_refs();

    println!("\n===== 7.  DANGLING REFERENCES =====");
    demo_dangling();

    println!("\n===== 8.  NLL =====");
    demo_nll();

    println!("\n===== 9.  SLICES =====");
    demo_slices();

    println!("\n===== 10. OWNERSHIP IN FUNCTIONS =====");
    demo_ownership_in_fns();

    println!("\n===== 11. BORROWING IN FUNCTIONS =====");
    demo_borrowing_in_fns();

    println!("\n===== 12. ARC =====");
    demo_arc();
}
