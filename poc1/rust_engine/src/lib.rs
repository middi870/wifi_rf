mod filters;
mod fft;
mod kalman;
mod butterworth;

use pyo3::prelude::*;
use pyo3::types::PyModule;

#[pymodule]
fn wifi_core(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(filters::ewma, m)?)?;
    m.add_function(wrap_pyfunction!(fft::fft_power, m)?)?;

    m.add_function(wrap_pyfunction!(kalman::kalman, m)?)?;
    m.add_function(wrap_pyfunction!(butterworth::butterworth, m)?)?;

    Ok(())
}