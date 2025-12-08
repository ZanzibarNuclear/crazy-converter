pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "newton" | "newtons" | "n" => Ok(value),
        "pound-force" | "pounds-force" | "lbf" | "lb-f" => Ok(value * 4.44822),
        "kilogram-force" | "kilograms-force" | "kgf" | "kg-f" => Ok(value * 9.80665),
        "dyne" | "dynes" | "dyn" => Ok(value * 0.00001),
        _ => Err(format!("Unknown force unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "newton" | "newtons" | "n" => Ok(value),
        "pound-force" | "pounds-force" | "lbf" | "lb-f" => Ok(value / 4.44822),
        "kilogram-force" | "kilograms-force" | "kgf" | "kg-f" => Ok(value / 9.80665),
        "dyne" | "dynes" | "dyn" => Ok(value / 0.00001),
        _ => Err(format!("Unknown force unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_force_conversions() {
        assert!((convert(1.0, "kilogram-force", "newtons").unwrap() - 9.80665).abs() < 0.0001);
    }
}

