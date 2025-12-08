pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "meter per second" | "meters per second" | "m/s" | "ms^-1" => Ok(value),
        "kilometer per hour" | "kilometers per hour" | "km/h" | "kmh" | "kph" => Ok(value / 3.6),
        "mile per hour" | "miles per hour" | "mph" | "mi/h" => Ok(value * 0.44704),
        "foot per second" | "feet per second" | "ft/s" | "fps" => Ok(value * 0.3048),
        "knot" | "knots" | "kt" | "kn" => Ok(value * 0.514444),
        _ => Err(format!("Unknown speed unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "meter per second" | "meters per second" | "m/s" | "ms^-1" => Ok(value),
        "kilometer per hour" | "kilometers per hour" | "km/h" | "kmh" | "kph" => Ok(value * 3.6),
        "mile per hour" | "miles per hour" | "mph" | "mi/h" => Ok(value / 0.44704),
        "foot per second" | "feet per second" | "ft/s" | "fps" => Ok(value / 0.3048),
        "knot" | "knots" | "kt" | "kn" => Ok(value / 0.514444),
        _ => Err(format!("Unknown speed unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_conversions() {
        assert!((convert(1.0, "m/s", "km/h").unwrap() - 3.6).abs() < 0.0001);
        assert!((convert(60.0, "mph", "km/h").unwrap() - 96.56064).abs() < 0.01);
    }
}

