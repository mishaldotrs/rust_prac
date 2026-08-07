use super::*;

pub fn run() {
    println!("\n===== 1.  PANIC =====");
    let v = vec![1, 2, 3];
    println!("element: {}", get_element(&v, 1));

    println!("\n===== 2.  OPTION =====");
    println!("{:?}", is_sqrt(9.0));
    println!("{:?}", is_sqrt(-4.0));

    println!("\n===== 3.  RESULT =====");
    println!("{:?}", divide(10.0, 2.0));
    println!("{:?}", divide(10.0, 0.0));

    println!("\n===== 4.  UNWRAP =====");
    demo_unwrap();

    println!("\n===== 5.  EXPECT =====");
    demo_expect();

    println!("\n===== 6.  MATCH =====");
    demo_match();

    println!("\n===== 7.  ? OPERATOR =====");
    demo_question_mark();

    println!("\n===== 8.  CUSTOM ERROR =====");
    demo_custom_error();

    println!("\n===== 9.  DISPLAY + ERROR TRAIT =====");
    demo_display_error();

    println!("\n===== 10. FROM TRAIT =====");
    demo_from();

    println!("\n===== 11. BOX<DYN ERROR> =====");
    demo_box_error();

    println!("\n===== 12. THISERROR =====");
    demo_thiserror();

    println!("\n===== 13. ANYHOW =====");
    demo_anyhow();

    println!("\n===== 14. AXUM ERRORS =====");
    println!("see comment in mod.rs — requires axum crate");
}
