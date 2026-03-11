use pyo3::prelude::*;

#[pyfunction]
pub fn ewma(Signal: Vec<f64>, alpha: f64) -> Vec<f64> {
    let mut result = Vec::with_capacity(Signal.len());

    let mut prev = Signal[0];
    result.push(prev)

    for &x in &signal[1..] {
        prev = alpha *x + (1.0 - alpha) * prev;
        result.push(prev)
    }
    result
}