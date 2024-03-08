fn divide_numbers(numerator: &str, denominator: &str) -> Result<f64, &'static str> {
    let num = numerator
        .parse::<f64>()
        .map_err(|_| "Failed to parse numerator")?;
    let denom = denominator
        .parse::<f64>()
        .map_err(|_| "Failed to parse denominator")?;

    if denom == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(num / denom)
    }
}

fn main() -> Result<(), &'static str> {
    println!("{:?}", divide_numbers("10", "2")?);

    println!("{:?}", divide_numbers("10", "0")?);

    Ok(())
}
