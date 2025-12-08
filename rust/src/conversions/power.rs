pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "watt" | "watts" | "w" => Ok(value),
        "kilowatt" | "kilowatts" | "kw" => Ok(value * 1000.0),
        "megawatt" | "megawatts" | "mw" => Ok(value * 1000000.0),
        "horsepower" | "hp" | "mechanical horsepower" => Ok(value * 745.7),
        "metric horsepower" | "ps" => Ok(value * 735.499),
        "btu per hour" | "btu/h" | "btuh" => Ok(value * 0.293071),
        "foot-pound per second" | "ft-lb/s" | "ft·lb/s" => Ok(value * 1.35582),
        _ => Err(format!("Unknown power unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "watt" | "watts" | "w" => Ok(value),
        "kilowatt" | "kilowatts" | "kw" => Ok(value / 1000.0),
        "megawatt" | "megawatts" | "mw" => Ok(value / 1000000.0),
        "horsepower" | "hp" | "mechanical horsepower" => Ok(value / 745.7),
        "metric horsepower" | "ps" => Ok(value / 735.499),
        "btu per hour" | "btu/h" | "btuh" => Ok(value / 0.293071),
        "foot-pound per second" | "ft-lb/s" | "ft·lb/s" => Ok(value / 1.35582),
        _ => Err(format!("Unknown power unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_conversions() {
        assert!((convert(1.0, "hp", "watts").unwrap() - 745.7).abs() < 0.1);
        assert!((convert(1000.0, "watts", "kilowatts").unwrap() - 1.0).abs() < 0.0001);
    }
}

