pub struct RingBuffer {
    data: Vec<f64>,
    index: usize,
    filled: bool,
}

impl RingBuffer {
    pub fn new(size: usize) -> Self {
        Self{
            data: vec![0.0; size],
            index: 0,
            filled: false,
        }
    }

    pub fn push(&mut self, value: f64) {
        self.data[self.index] = value;
        self.index += 1;

        if self.index >= self.data.len() {
            self.index = 0;
            self.filled = true;
        }
    }

    pub fn ready(&self) -> bool {
        self.filled
    }

    pub fn snapshot(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.data.len());

        let n = self.data.len();

        for i in 0..n {
            let idx = (self.index + i) % n;
            out.push(self.data[idx]);
        }
        out
    }
}