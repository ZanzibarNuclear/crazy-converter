pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "pascal" | "pascals" | "pa" => Ok(value),
        "kilopascal" | "kilopascals" | "kpa" => Ok(value * 1000.0),
        "megapascal" | "megapascals" | "mpa" => Ok(value * 1000000.0),
        "pound per square inch" | "pounds per square inch" | "psi" | "lb/in²" | "lb/in2" => Ok(value * 6894.76),
        "bar" | "bars" => Ok(value * 100000.0),
        "atmosphere" | "atmospheres" | "atm" => Ok(value * 101325.0),
        "torr" => Ok(value * 133.322),
        "millimeter of mercury" | "millimeters of mercury" | "mmhg" | "mm hg" => Ok(value * 133.322),
        _ => Err(format!("Unknown pressure unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "pascal" | "pascals" | "pa" => Ok(value),
        "kilopascal" | "kilopascals" | "kpa" => Ok(value / 1000.0),
        "megapascal" | "megapascals" | "mpa" => Ok(value / 1000000.0),
        "pound per square inch" | "pounds per square inch" | "psi" | "lb/in²" | "lb/in2" => Ok(value / 6894.76),
        "bar" | "bars" => Ok(value / 100000.0),
        "atmosphere" | "atmospheres" | "atm" => Ok(value / 101325.0),
        "torr" => Ok(value / 133.322),
        "millimeter of mercury" | "millimeters of mercury" | "mmhg" | "mm hg" => Ok(value / 133.322),
        _ => Err(format!("Unknown pressure unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_conversions() {
        assert!((convert(1.0, "bar", "pascals").unwrap() - 100000.0).abs() < 0.01);
        assert!((convert(1.0, "atm", "psi").unwrap() - 14.6959).abs() < 0.1);
    }
}

