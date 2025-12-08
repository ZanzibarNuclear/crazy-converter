pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "square meter" | "square meters" | "m²" | "m2" | "sq m" | "sqm" => Ok(value),
        "square centimeter" | "square centimeters" | "cm²" | "cm2" | "sq cm" => Ok(value * 0.0001),
        "square inch" | "square inches" | "in²" | "in2" | "sq in" => Ok(value * 0.00064516),
        "square foot" | "square feet" | "ft²" | "ft2" | "sq ft" => Ok(value * 0.092903),
        "square mile" | "square miles" | "mi²" | "mi2" | "sq mi" => Ok(value * 2589988.11),
        "square kilometer" | "square kilometers" | "km²" | "km2" | "sq km" => Ok(value * 1000000.0),
        "acre" | "acres" | "ac" => Ok(value * 4046.86),
        "hectare" | "hectares" | "ha" => Ok(value * 10000.0),
        _ => Err(format!("Unknown area unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "square meter" | "square meters" | "m²" | "m2" | "sq m" | "sqm" => Ok(value),
        "square centimeter" | "square centimeters" | "cm²" | "cm2" | "sq cm" => Ok(value / 0.0001),
        "square inch" | "square inches" | "in²" | "in2" | "sq in" => Ok(value / 0.00064516),
        "square foot" | "square feet" | "ft²" | "ft2" | "sq ft" => Ok(value / 0.092903),
        "square mile" | "square miles" | "mi²" | "mi2" | "sq mi" => Ok(value / 2589988.11),
        "square kilometer" | "square kilometers" | "km²" | "km2" | "sq km" => Ok(value / 1000000.0),
        "acre" | "acres" | "ac" => Ok(value / 4046.86),
        "hectare" | "hectares" | "ha" => Ok(value / 10000.0),
        _ => Err(format!("Unknown area unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_conversions() {
        assert!((convert(1.0, "hectare", "square meters").unwrap() - 10000.0).abs() < 0.01);
        assert!((convert(1.0, "acre", "square meters").unwrap() - 4046.86).abs() < 0.1);
    }
}

