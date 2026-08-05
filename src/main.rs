mod errors;

fn main() {
    match errors::divide(10.0, 3.3) {
        Ok(val) => println!("result: {}", val),
        Err(e) => println!("error: {}", e),
    }
}
