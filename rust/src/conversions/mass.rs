pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "kilogram" | "kilograms" | "kg" => Ok(value),
        "gram" | "grams" | "g" => Ok(value * 0.001),
        "milligram" | "milligrams" | "mg" => Ok(value * 0.000001),
        "pound" | "pounds" | "lb" | "lbs" => Ok(value * 0.453592),
        "ounce" | "ounces" | "oz" => Ok(value * 0.0283495),
        "ton" | "tons" | "t" => Ok(value * 1000.0), // Metric ton
        "ton (metric)" | "metric ton" | "metric tons" => Ok(value * 1000.0),
        "ton (us)" | "us ton" | "short ton" | "short tons" => Ok(value * 907.185),
        "stone" | "stones" => Ok(value * 6.35029),
        _ => Err(format!("Unknown mass unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "kilogram" | "kilograms" | "kg" => Ok(value),
        "gram" | "grams" | "g" => Ok(value / 0.001),
        "milligram" | "milligrams" | "mg" => Ok(value / 0.000001),
        "pound" | "pounds" | "lb" | "lbs" => Ok(value / 0.453592),
        "ounce" | "ounces" | "oz" => Ok(value / 0.0283495),
        "ton" | "tons" | "t" => Ok(value / 1000.0),
        "ton (metric)" | "metric ton" | "metric tons" => Ok(value / 1000.0),
        "ton (us)" | "us ton" | "short ton" | "short tons" => Ok(value / 907.185),
        "stone" | "stones" => Ok(value / 6.35029),
        _ => Err(format!("Unknown mass unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_conversions() {
        assert!((convert(1000.0, "grams", "kilograms").unwrap() - 1.0).abs() < 0.0001);
        assert!((convert(1.0, "pound", "kilograms").unwrap() - 0.453592).abs() < 0.0001);
    }
}

