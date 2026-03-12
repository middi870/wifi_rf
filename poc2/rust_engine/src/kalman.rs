pub fn kalman(signal: &[f64], q: f64, r: f64) -> Vec<f64> {
    let mut x = signal[0];
    let mut p = 1.0;

    let mut result = Vec::with_capacity(signal.len());
    result.push(x);

    for &z in &signal[1..] {
        p += q;

        let k = p / (p + r);
        x = x + k * (z - x);
        p = (1.0 - k) * p;

        result.push(x);
    }
    result
}