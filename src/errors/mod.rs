pub fn divide<T>(a: T, b: T) -> Result<T, String>
where
    T: std::ops::Div<Output = T> + PartialEq + Default,
{
    if b == T::default() {   // T::default() = 0 for i32, 0.0 for f64
        Err(String::from("denominator cannot be zero"))
    } else {
        Ok(a / b)
    }
}