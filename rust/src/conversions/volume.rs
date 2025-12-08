pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "liter" | "liters" | "l" | "L" => Ok(value * 0.001),
        "milliliter" | "milliliters" | "ml" | "mL" => Ok(value * 0.000001),
        "cubic meter" | "cubic meters" | "m³" | "m3" => Ok(value),
        "cubic centimeter" | "cubic centimeters" | "cm³" | "cm3" | "cc" => Ok(value * 0.000001),
        "cubic inch" | "cubic inches" | "in³" | "in3" => Ok(value * 0.000016387064),
        "cubic foot" | "cubic feet" | "ft³" | "ft3" => Ok(value * 0.028316846592),
        "gallon" | "gallons" | "gal" => Ok(value * 0.00378541), // US gallon
        "gallon (us)" | "us gallon" | "us gallons" => Ok(value * 0.00378541),
        "gallon (uk)" | "uk gallon" | "imperial gallon" | "imperial gallons" => Ok(value * 0.00454609),
        "fluid ounce" | "fluid ounces" | "fl oz" | "floz" => Ok(value * 0.0000295735), // US fluid ounce
        "cup" | "cups" => Ok(value * 0.000236588),
        "pint" | "pints" | "pt" => Ok(value * 0.000473176), // US pint
        "quart" | "quarts" | "qt" => Ok(value * 0.000946353), // US quart
        _ => Err(format!("Unknown volume unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "liter" | "liters" | "l" | "L" => Ok(value / 0.001),
        "milliliter" | "milliliters" | "ml" | "mL" => Ok(value / 0.000001),
        "cubic meter" | "cubic meters" | "m³" | "m3" => Ok(value),
        "cubic centimeter" | "cubic centimeters" | "cm³" | "cm3" | "cc" => Ok(value / 0.000001),
        "cubic inch" | "cubic inches" | "in³" | "in3" => Ok(value / 0.000016387064),
        "cubic foot" | "cubic feet" | "ft³" | "ft3" => Ok(value / 0.028316846592),
        "gallon" | "gallons" | "gal" => Ok(value / 0.00378541),
        "gallon (us)" | "us gallon" | "us gallons" => Ok(value / 0.00378541),
        "gallon (uk)" | "uk gallon" | "imperial gallon" | "imperial gallons" => Ok(value / 0.00454609),
        "fluid ounce" | "fluid ounces" | "fl oz" | "floz" => Ok(value / 0.0000295735),
        "cup" | "cups" => Ok(value / 0.000236588),
        "pint" | "pints" | "pt" => Ok(value / 0.000473176),
        "quart" | "quarts" | "qt" => Ok(value / 0.000946353),
        _ => Err(format!("Unknown volume unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_conversions() {
        assert!((convert(1000.0, "liters", "cubic meters").unwrap() - 1.0).abs() < 0.0001);
        assert!((convert(1.0, "gallon", "liters").unwrap() - 3.78541).abs() < 0.01);
    }
}

