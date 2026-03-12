mod acquisition;
mod buffer;
mod engine;
mod features;
mod fft;
mod filters;
mod kalman;

use pyo3::prelude::*;
use pyo3::types::PyModule;

use engine::Engine;

#[pyfunction]
fn start_engine(interface: String) {
    let mut engine = Engine::new(&interface, 128);

    engine.run();
}

#[pymodule]
fn rf_engine(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_engine, m)?)?;
    
    Ok(())
}