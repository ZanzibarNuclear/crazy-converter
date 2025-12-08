mod conversions;

use pyo3::prelude::*;

use conversions::{
    convert_time, convert_length, convert_area, convert_volume, convert_mass,
    convert_speed, convert_acceleration, convert_force, convert_pressure,
    convert_energy, convert_power, convert_momentum, convert_torque,
    convert_temperature,
};

/// A Python module for unit conversions
#[pymodule]
fn converterator_rust(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_time, m)?)?;
    m.add_function(wrap_pyfunction!(convert_length, m)?)?;
    m.add_function(wrap_pyfunction!(convert_area, m)?)?;
    m.add_function(wrap_pyfunction!(convert_volume, m)?)?;
    m.add_function(wrap_pyfunction!(convert_mass, m)?)?;
    m.add_function(wrap_pyfunction!(convert_speed, m)?)?;
    m.add_function(wrap_pyfunction!(convert_acceleration, m)?)?;
    m.add_function(wrap_pyfunction!(convert_force, m)?)?;
    m.add_function(wrap_pyfunction!(convert_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(convert_energy, m)?)?;
    m.add_function(wrap_pyfunction!(convert_power, m)?)?;
    m.add_function(wrap_pyfunction!(convert_momentum, m)?)?;
    m.add_function(wrap_pyfunction!(convert_torque, m)?)?;
    m.add_function(wrap_pyfunction!(convert_temperature, m)?)?;
    Ok(())
}
