pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "newton-meter" | "newton-meters" | "n·m" | "n m" | "nm" => Ok(value),
        "pound-foot" | "pound-feet" | "lb·ft" | "lb ft" | "lbft" => Ok(value * 1.35582),
        "pound-inch" | "pound-inches" | "lb·in" | "lb in" | "lbin" => Ok(value * 0.112985),
        "kilogram-force-meter" | "kgf·m" | "kgf m" => Ok(value * 9.80665),
        _ => Err(format!("Unknown torque unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "newton-meter" | "newton-meters" | "n·m" | "n m" | "nm" => Ok(value),
        "pound-foot" | "pound-feet" | "lb·ft" | "lb ft" | "lbft" => Ok(value / 1.35582),
        "pound-inch" | "pound-inches" | "lb·in" | "lb in" | "lbin" => Ok(value / 0.112985),
        "kilogram-force-meter" | "kgf·m" | "kgf m" => Ok(value / 9.80665),
        _ => Err(format!("Unknown torque unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torque_conversions() {
        assert!((convert(1.0, "lb·ft", "n·m").unwrap() - 1.35582).abs() < 0.0001);
    }
}

