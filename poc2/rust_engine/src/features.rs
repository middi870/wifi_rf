pub struct Features {
    pub variance: f64,
    pub energy: f64,
    pub motion: f64,
}

pub fn extract(signal: &[f64]) -> Features {
    let n = signal.len() as f64;
    let mean = signal.iter().sum::<f64>() / n;

    let variance = signal.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    let motion = variance.sqrt();

    let energy = signal.iter().map(|x| x * x).sum::<f64>() / signal.len() as f64;

    Features {
        variance,
        energy,
        motion,
    }
}