use pyo3::prelude::*;
use rustfft{FftPlanner, num_complex::Complex};

#[pyfunction]
pub fn fft_power(signal: Vec<f64>) -> Vec<f64> {
    
}