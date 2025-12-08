pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let base_value = from(value, from_unit)?;
    to(base_value, to_unit)
}

fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "joule" | "joules" | "j" => Ok(value),
        "kilojoule" | "kilojoules" | "kj" => Ok(value * 1000.0),
        "megajoule" | "megajoules" | "mj" => Ok(value * 1000000.0),
        "kilowatt-hour" | "kilowatt-hours" | "kwh" | "kw·h" => Ok(value * 3600000.0),
        "calorie" | "calories" | "cal" => Ok(value * 4.184),
        "kilocalorie" | "kilocalories" | "kcal" => Ok(value * 4184.0),
        "btu" | "british thermal unit" | "british thermal units" => Ok(value * 1055.06),
        "foot-pound" | "foot-pounds" | "ft-lb" | "ft·lb" => Ok(value * 1.35582),
        "electronvolt" | "electronvolts" | "ev" => Ok(value * 1.602176634e-19),
        "kiloelectronvolt" | "kiloelectronvolts" | "kev" => Ok(value * 1.602176634e-16),
        "megaelectronvolt" | "megaelectronvolts" | "mev" => Ok(value * 1.602176634e-13),
        _ => Err(format!("Unknown energy unit: {}", unit)),
    }
}

fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "joule" | "joules" | "j" => Ok(value),
        "kilojoule" | "kilojoules" | "kj" => Ok(value / 1000.0),
        "megajoule" | "megajoules" | "mj" => Ok(value / 1000000.0),
        "kilowatt-hour" | "kilowatt-hours" | "kwh" | "kw·h" => Ok(value / 3600000.0),
        "calorie" | "calories" | "cal" => Ok(value / 4.184),
        "kilocalorie" | "kilocalories" | "kcal" => Ok(value / 4184.0),
        "btu" | "british thermal unit" | "british thermal units" => Ok(value / 1055.06),
        "foot-pound" | "foot-pounds" | "ft-lb" | "ft·lb" => Ok(value / 1.35582),
        "electronvolt" | "electronvolts" | "ev" => Ok(value / 1.602176634e-19),
        "kiloelectronvolt" | "kiloelectronvolts" | "kev" => Ok(value / 1.602176634e-16),
        "megaelectronvolt" | "megaelectronvolts" | "mev" => Ok(value / 1.602176634e-13),
        _ => Err(format!("Unknown energy unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_conversions() {
        assert!((convert(1.0, "kcal", "joules").unwrap() - 4184.0).abs() < 0.01);
        assert!((convert(1.0, "kwh", "joules").unwrap() - 3600000.0).abs() < 0.01);
    }
}

