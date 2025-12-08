pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "meter" | "meters" | "m" => Ok(value),
        "centimeter" | "centimeters" | "cm" => Ok(value * 0.01),
        "millimeter" | "millimeters" | "mm" => Ok(value * 0.001),
        "micrometer" | "micrometers" | "μm" | "um" => Ok(value * 1e-6),
        "nanometer" | "nanometers" | "nm" => Ok(value * 1e-9),
        "kilometer" | "kilometers" | "km" => Ok(value * 1000.0),
        "inch" | "inches" | "in" | "\"" => Ok(value * 0.0254),
        "foot" | "feet" | "ft" | "'" => Ok(value * 0.3048),
        "yard" | "yards" | "yd" => Ok(value * 0.9144),
        "mile" | "miles" | "mi" => Ok(value * 1609.344),
        _ => Err(format!("Unknown length unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "meter" | "meters" | "m" => Ok(value),
        "centimeter" | "centimeters" | "cm" => Ok(value / 0.01),
        "millimeter" | "millimeters" | "mm" => Ok(value / 0.001),
        "micrometer" | "micrometers" | "μm" | "um" => Ok(value / 1e-6),
        "nanometer" | "nanometers" | "nm" => Ok(value / 1e-9),
        "kilometer" | "kilometers" | "km" => Ok(value / 1000.0),
        "inch" | "inches" | "in" | "\"" => Ok(value / 0.0254),
        "foot" | "feet" | "ft" | "'" => Ok(value / 0.3048),
        "yard" | "yards" | "yd" => Ok(value / 0.9144),
        "mile" | "miles" | "mi" => Ok(value / 1609.344),
        _ => Err(format!("Unknown length unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_conversions() {
        assert!((convert(100.0, "centimeters", "meters").unwrap() - 1.0).abs() < 0.0001);
        assert!((convert(1.0, "mile", "kilometers").unwrap() - 1.609344).abs() < 0.0001);
    }
}

