mod errors;

fn main() {
    println!("\n===== 1. PANIC =====");
    let v = vec![1, 2, 3];
    println!("element: {}", errors::get_element(&v, 1));
    // errors::get_element(&v, 10); // uncomment to see panic

    println!("\n===== 2. OPTION =====");
    println!("{:?}", errors::is_sqrt(9.0));
    println!("{:?}", errors::is_sqrt(-4.0));

    println!("\n===== 3. RESULT =====");
    println!("{:?}", errors::divide(10.0, 2.0));
    println!("{:?}", errors::divide(10.0, 0.0));

    println!("\n===== 4. UNWRAP =====");
    errors::demo_unwrap();

    println!("\n===== 5. EXPECT =====");
    errors::demo_expect();

    println!("\n===== 6. MATCH =====");
    errors::demo_match();

    println!("\n===== 7. ? OPERATOR =====");
    errors::demo_question_mark();

    println!("\n===== 8. CUSTOM ERROR =====");
    errors::demo_custom_error();

    println!("\n===== 9. DISPLAY + ERROR TRAIT =====");
    errors::demo_display_error();

    println!("\n===== 10. FROM TRAIT =====");
    errors::demo_from();

    println!("\n===== 11. BOX<DYN ERROR> =====");
    errors::demo_box_error();

    println!("\n===== 12. THISERROR =====");
    errors::demo_thiserror();

    println!("\n===== 13. ANYHOW =====");
    errors::demo_anyhow();

    println!("\n===== 14. AXUM ERRORS =====");
    println!("see the comment in src/errors/mod.rs — requires axum crate");
}
