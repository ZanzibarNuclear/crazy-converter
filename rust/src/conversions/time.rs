/// Convert a value from one time unit to another.
///
/// This function uses a two-step conversion process:
/// 1. First, convert the value from the source unit to the base unit (seconds)
/// 2. Then, convert from the base unit to the target unit
///
/// This approach allows us to convert between any two units without needing
/// conversion factors for every possible pair. We only need conversion factors
/// from each unit to/from the base unit (seconds).
///
/// # Arguments
///
/// * `value` - The numerical value to convert
/// * `from_unit` - The source unit (e.g., "minutes", "hours", "days")
/// * `to_unit` - The target unit (e.g., "seconds", "weeks", "years")
///
/// # Returns
///
/// * `Ok(f64)` - The converted value
/// * `Err(String)` - An error message if an unknown unit is provided
///
/// # Example
///
/// ```
/// // Convert 60 seconds to 1 minute
/// let result = convert(60.0, "seconds", "minutes").unwrap();
/// assert_eq!(result, 1.0);
/// ```
pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    // Step 1: Convert from source unit to base unit (seconds)
    let base_value = from(value, from_unit)?;
    // Step 2: Convert from base unit to target unit
    to(base_value, to_unit)
}

/// Convert a value from any time unit to the base unit (seconds).
///
/// This is the first step in the conversion process. All time units
/// are converted to seconds as the common base unit.
fn from(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "second" | "seconds" | "s" | "sec" => Ok(value),
        "minute" | "minutes" | "min" => Ok(value * 60.0),
        "hour" | "hours" | "h" | "hr" | "hrs" => Ok(value * 3600.0),
        "day" | "days" | "d" => Ok(value * 86400.0),
        "week" | "weeks" | "w" => Ok(value * 604800.0),
        "month" | "months" | "mo" => Ok(value * 2629746.0), // Average month (30.44 days)
        "year" | "years" | "y" | "yr" | "yrs" => Ok(value * 31556952.0), // Average year (365.25 days)
        _ => Err(format!("Unknown time unit: {}", unit)),
    }
}

/// Convert a value from the base unit (seconds) to any time unit.
///
/// This is the second step in the conversion process. The value in seconds
/// is converted to the desired target unit.
fn to(value: f64, unit: &str) -> Result<f64, String> {
    let normalized = unit.to_lowercase();
    match normalized.as_str() {
        "second" | "seconds" | "s" | "sec" => Ok(value),
        "minute" | "minutes" | "min" => Ok(value / 60.0),
        "hour" | "hours" | "h" | "hr" | "hrs" => Ok(value / 3600.0),
        "day" | "days" | "d" => Ok(value / 86400.0),
        "week" | "weeks" | "w" => Ok(value / 604800.0),
        "month" | "months" | "mo" => Ok(value / 2629746.0),
        "year" | "years" | "y" | "yr" | "yrs" => Ok(value / 31556952.0),
        _ => Err(format!("Unknown time unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_conversions() {
        assert!((convert(60.0, "seconds", "minutes").unwrap() - 1.0).abs() < 0.0001);
        assert!((convert(1.0, "hour", "minutes").unwrap() - 60.0).abs() < 0.0001);
        assert!((convert(1.0, "day", "hours").unwrap() - 24.0).abs() < 0.0001);
    }
}

