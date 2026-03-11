use pyo3::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};

#[pyfunction]
pub fn fft_power(signal: Vec<f64>) -> Vec<f64> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(signal.len());

    let mut buffer: Vec<Complex<f64>> = signal.into_iter().map(|x| Complex { re: x, im: 0.0 }).collect();
    fft.process(&mut buffer);

    buffer.iter().map(|c| c.norm()).collect()
}