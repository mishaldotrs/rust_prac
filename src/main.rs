#[path = "enums-pattern_matching/mod.rs"]
mod enums_pattern_matching;
mod errors;
fn main(){
    println!("\n===== ENUMS 1. IF LET =====");
    enums_pattern_matching::demo_if_let();

    println!("\n===== ENUMS 2. DESTRUCTURING =====");
    enums_pattern_matching::demo_destructuring();

    println!("\n===== ENUMS 3. STATE MACHINE =====");
    enums_pattern_matching::demo_state_machine();

    println!("\n===== ENUMS 4. RESULT HANDLING =====");
    enums_pattern_matching::demo_result_handling();

    println!("\n===== ENUMS 5. API DESIGN =====");
    enums_pattern_matching::demo_api_design();
}
