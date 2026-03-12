pub fn ewma(signal: &[f64], alpha: f64) -> Vec<f64> {
    let mut out =  Vec::with_capacity(signal.len());

    let mut prev = signal[0];
    out.push(prev);

    for &x in &signal[1..]{
        let y = alpha * x + (1.0 - alpha) * prev;

        out.push(y);
        prev = y;
    }
    out
}

// pub fn butterworth(signal: &[f64], alpha: f64) -> Vec<f64> {
//     let mut out = Vec::with_capacity(signal.len());

//     let mut prev = signal[0];
//     out.push(prev);

//     for &x in &signal[1..] {
//         let y = alpha * x + (1.0 - alpha) * prev;

//         out.push(y);
//         prev = y;
//     }
//     out
// }


pub fn butterworth(signal: &[f64], cutoff: f64, sample_rate: f64) -> Vec<f64> {
    if signal.is_empty() {
        return Vec::new();
    }

    let ita = 1.0 / (std::f64::consts::PI * cutoff / sample_rate).tan();
    let q = 2.0_f64.sqrt();

    let b0 = 1.0 / (1.0 + q * ita + ita * ita);
    let b1 = 2.0 * b0;
    let b2 = b0;

    let a1 = 2.0 * (ita * ita - 1.0) * b0;
    let a2 = -(1.0 - q * ita + ita * ita) * b0;

    let mut out = Vec::with_capacity(signal.len());

    let mut x1 = 0.0;
    let mut x2 = 0.0;
    let mut y1 = 0.0;
    let mut y2 = 0.0;

    for &x0 in signal {
        let y0 = b0 * x0 + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;

        out.push(y0);

        x2 = x1;
        x1 = x0;

        y2 = y1;
        y1 = y0;
    }

    out
}