pub mod time;
pub mod length;
pub mod area;
pub mod volume;
pub mod mass;
pub mod speed;
pub mod acceleration;
pub mod force;
pub mod pressure;
pub mod energy;
pub mod power;
pub mod momentum;
pub mod torque;
pub mod temperature;

use pyo3::prelude::*;

#[pyfunction]
pub fn convert_time(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    time::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_length(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    length::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_area(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    area::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_volume(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    volume::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_mass(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    mass::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_speed(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    speed::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_acceleration(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    acceleration::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_force(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    force::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_pressure(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    pressure::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_energy(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    energy::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_power(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    power::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_momentum(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    momentum::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_torque(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    torque::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

#[pyfunction]
pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> PyResult<f64> {
    temperature::convert(value, from_unit, to_unit)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
}

