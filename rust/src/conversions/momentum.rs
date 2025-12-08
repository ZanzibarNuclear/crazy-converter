pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "kilogram-meter per second" | "kg·m/s" | "kg m/s" | "kgm/s" => Ok(value),
        "pound-foot per second" | "lb·ft/s" | "lb ft/s" | "lbft/s" => Ok(value * 0.138255),
        "newton-second" | "newton-seconds" | "n·s" | "n s" => Ok(value),
        _ => Err(format!("Unknown momentum unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "kilogram-meter per second" | "kg·m/s" | "kg m/s" | "kgm/s" => Ok(value),
        "pound-foot per second" | "lb·ft/s" | "lb ft/s" | "lbft/s" => Ok(value / 0.138255),
        "newton-second" | "newton-seconds" | "n·s" | "n s" => Ok(value),
        _ => Err(format!("Unknown momentum unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_momentum_conversions() {
        assert!((convert(1.0, "n·s", "kg·m/s").unwrap() - 1.0).abs() < 0.0001);
    }
}

