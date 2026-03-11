use pyo3::prelude::*;

#[pyfunction]
pub fn butterworth(signal: Vec<f64>, alpha: f64) -> Vec<f64> {
    let mut output = Vec::with_capacity(signal.len());
    let mut prev = signal[0];

    for &x in signal[1..] {
        let filtered = alpha * x + (1.0 - alpha) * prev;

        output.push(filtered);
        prev = filtered;
    }

    output

}