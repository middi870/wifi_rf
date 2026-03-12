use crate::acquisition::read_rssi;
use crate::buffer::RingBuffer;
use crate::features::extract;
use crate::fft::fft_power;
use crate::filters::{ewma, butterworth};
use crate::kalman::kalman;

use std::thread;
use std::time::Duration;

pub struct Engine {
    buffer: RingBuffer,
    interface: String,
}

impl Engine {
    pub fn new(interface: &str, size: usize) -> Self {
        Self {
            buffer: RingBuffer::new(size),
            interface: interface.to_string(),
        }
    }

    pub fn step(&mut self) {
        if let Some(rssi)  = read_rssi(&self.interface) {
            self.buffer.push(rssi);

            if !self.buffer.ready() {
                return;
            }

            let signal = self.buffer.snapshot();
            let window = &signal[signal.len() - 64..];

            let butter = butterworth(window, 0.2, 10.0);
            let kal = kalman(&butter, 0.01, 2.0);
            let smooth = ewma(&kal, 0.3);
            let spectrum = fft_power(&smooth);
            let features = extract(&spectrum);

            println!(
                "motion {:.3} energy {:.3} variance {:.3}",
                features.motion, features.energy, features.variance
            );
        }
    }

    pub fn run(&mut self) {
        loop{
            self.step();

            thread::sleep(Duration::from_millis(100));
        }
    }
}