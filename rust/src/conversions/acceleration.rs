pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "meter per second squared" | "meters per second squared" | "m/s²" | "m/s2" | "ms^-2" => Ok(value),
        "foot per second squared" | "feet per second squared" | "ft/s²" | "ft/s2" | "fps2" => Ok(value * 0.3048),
        "standard gravity" | "g" | "g-force" => Ok(value * 9.80665),
        _ => Err(format!("Unknown acceleration unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "meter per second squared" | "meters per second squared" | "m/s²" | "m/s2" | "ms^-2" => Ok(value),
        "foot per second squared" | "feet per second squared" | "ft/s²" | "ft/s2" | "fps2" => Ok(value / 0.3048),
        "standard gravity" | "g" | "g-force" => Ok(value / 9.80665),
        _ => Err(format!("Unknown acceleration unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceleration_conversions() {
        assert!((convert(1.0, "g", "m/s2").unwrap() - 9.80665).abs() < 0.0001);
    }
}

