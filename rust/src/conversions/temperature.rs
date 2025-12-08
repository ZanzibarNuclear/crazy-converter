/// Convert a value from one temperature unit to another.
///
/// This function uses a two-step conversion process:
/// 1. First, convert the value from the source unit to the base unit (Kelvin)
/// 2. Then, convert from the base unit to the target unit
///
/// Temperature conversions are different from other unit conversions because
/// they involve both scaling factors and offsets. Kelvin is used as the base unit
/// since it's the SI unit and has no offset.
///
/// # Arguments
///
/// * `value` - The numerical value to convert
/// * `from_unit` - The source unit (e.g., "celsius", "fahrenheit", "kelvin")
/// * `to_unit` - The target unit (e.g., "celsius", "fahrenheit", "kelvin", "rankine")
///
/// # Returns
///
/// * `Ok(f64)` - The converted value
/// * `Err(String)` - An error message if an unknown unit is provided
///
/// # Example
///
/// ```
/// // Convert 0°C to 32°F
/// let result = convert(0.0, "celsius", "fahrenheit").unwrap();
/// assert_eq!(result, 32.0);
/// ```
pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    // Step 1: Convert from source unit to base unit (Kelvin)
    let base_value = from(value, from_unit)?;
    // Step 2: Convert from base unit to target unit
    to(base_value, to_unit)
}

/// Convert a value from any temperature unit to the base unit (Kelvin).
///
/// This is the first step in the conversion process. All temperature units
/// are converted to Kelvin as the common base unit.
fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "kelvin" | "k" => Ok(value),
        "celsius" | "c" | "°c" | "deg c" | "degree celsius" | "degrees celsius" => {
            Ok(value + 273.15)
        }
        "fahrenheit" | "f" | "°f" | "deg f" | "degree fahrenheit" | "degrees fahrenheit" => {
            // Convert Fahrenheit to Celsius first, then to Kelvin
            let celsius = (value - 32.0) * 5.0 / 9.0;
            Ok(celsius + 273.15)
        }
        "rankine" | "r" | "°r" | "deg r" | "degree rankine" | "degrees rankine" => {
            // Rankine to Kelvin: K = °R × 5/9
            Ok(value * 5.0 / 9.0)
        }
        _ => Err(format!("Unknown temperature unit: {}", unit)),
    }
}

/// Convert a value from the base unit (Kelvin) to any temperature unit.
///
/// This is the second step in the conversion process. The value in Kelvin
/// is converted to the desired target unit.
fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "kelvin" | "k" => Ok(value),
        "celsius" | "c" | "°c" | "deg c" | "degree celsius" | "degrees celsius" => {
            Ok(value - 273.15)
        }
        "fahrenheit" | "f" | "°f" | "deg f" | "degree fahrenheit" | "degrees fahrenheit" => {
            // Convert Kelvin to Celsius first, then to Fahrenheit
            let celsius = value - 273.15;
            Ok(celsius * 9.0 / 5.0 + 32.0)
        }
        "rankine" | "r" | "°r" | "deg r" | "degree rankine" | "degrees rankine" => {
            // Kelvin to Rankine: °R = K × 9/5
            Ok(value * 9.0 / 5.0)
        }
        _ => Err(format!("Unknown temperature unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_conversions() {
        // Freezing point of water: 0°C = 32°F = 273.15K
        assert!((convert(0.0, "celsius", "fahrenheit").unwrap() - 32.0).abs() < 0.0001);
        assert!((convert(0.0, "celsius", "kelvin").unwrap() - 273.15).abs() < 0.0001);
        assert!((convert(32.0, "fahrenheit", "celsius").unwrap() - 0.0).abs() < 0.0001);

        // Boiling point of water: 100°C = 212°F = 373.15K
        assert!((convert(100.0, "celsius", "fahrenheit").unwrap() - 212.0).abs() < 0.0001);
        assert!((convert(100.0, "celsius", "kelvin").unwrap() - 373.15).abs() < 0.0001);

        // Absolute zero: 0K = -273.15°C = -459.67°F
        assert!((convert(0.0, "kelvin", "celsius").unwrap() + 273.15).abs() < 0.0001);
        assert!((convert(0.0, "kelvin", "fahrenheit").unwrap() + 459.67).abs() < 0.01);

        // Rankine conversions
        assert!((convert(0.0, "kelvin", "rankine").unwrap() - 0.0).abs() < 0.0001);
        assert!((convert(273.15, "kelvin", "rankine").unwrap() - 491.67).abs() < 0.01);
    }
}

