# README.md

```markdown
# WiFi RF Motion Sensing (Rust + Python)

This project explores **RF sensing using WiFi signals**.  
The goal is to detect human movement by analyzing small fluctuations in **WiFi RSSI (Received Signal Strength Indicator)**.

When a person moves inside a room, their body affects the RF propagation paths of the WiFi signal.  
By applying **signal processing techniques**, those disturbances can be detected and analyzed.

This repository contains multiple **Proof of Concept (POC)** implementations that experiment with different architectures and signal pipelines.

---

## Key Idea

WiFi signals interact with the environment through **multipath propagation**.

```

Router → Walls → Objects → Human Body → Laptop WiFi Card

```

Movement changes the RF path, which creates measurable fluctuations in RSSI.

This project builds a **signal processing pipeline** to analyze those changes.

---

## Architecture

```

WiFi RSSI
↓
Rust Acquisition Engine
↓
Ring Buffer
↓
Butterworth Filter
↓
Kalman State Estimator
↓
EWMA Smoothing
↓
FFT Spectral Analysis
↓
Feature Extraction
↓
Python Interface / Visualization

```

Rust handles the **performance-critical signal processing**, while Python is used for **experimentation and visualization**.

---

# Repository Structure

```

.
├── poc1
│   ├── python_int
│   │   ├── breathing_monitor.py
│   │   ├── motion_monitor.py
│   │   ├── radar_monitor.py
│   │   ├── rssi_reader.py
│   │   └── test_core.py
│   └── rust_engine
│       ├── Cargo.toml
│       └── src
│           ├── butterworth.rs
│           ├── fft.rs
│           ├── filters.rs
│           ├── kalman.rs
│           └── lib.rs
│
├── poc2
│   ├── python_int
│   │   ├── run_engine.py
│   │   └── test_engine.py
│   └── rust_engine
│       ├── Cargo.toml
│       └── src
│           ├── acquisition.rs
│           ├── buffer.rs
│           ├── engine.rs
│           ├── features.rs
│           ├── fft.rs
│           ├── filters.rs
│           ├── kalman.rs
│           └── lib.rs
│
├── README.md
└── requirements.txt

```

---

# POC Overview

## POC1

Initial prototype exploring **RSSI signal processing**.

Features:
- RSSI acquisition from WiFi interface
- Butterworth filtering
- Kalman filtering
- EWMA smoothing
- FFT analysis
- motion detection experiments
- breathing detection experiments
- radar-style visualization

Python handles most of the experimentation.

---

## POC2

Second prototype focusing on **performance and architecture improvements**.

Major changes:
- Rust-based **real-time RF engine**
- ring buffer for signal history
- structured feature extraction
- improved modular architecture
- Python only used for **interface and visualization**

Pipeline:

```

RSSI → Rust Engine → Filters → FFT → Feature Extraction → Python

````

---

# Installation

## 1. Clone Repository

```bash
git clone https://github.com/yourusername/wifi_rf.git
cd wifi_rf
````

---

## 2. Create Python Environment

```bash
python -m venv venv
source venv/bin/activate
```

---

## 3. Install Python Dependencies

```bash
pip install -r requirements.txt
```

---

## 4. Install Rust

Make sure Rust is installed:

```bash
rustc --version
cargo --version
```

If not installed:

```
https://rustup.rs
```

---

# Build Rust Engine

Inside a POC directory:

```
cd poc2/rust_engine
maturin develop --release
```

This compiles the Rust engine and exposes it as a Python module.

---

# Run Example

```
cd poc2/python_int
python run_engine.py
```

Example output:

```
motion 295.131 energy 89260.079 variance 87102.191
motion 303.738 energy 94028.485 variance 92256.992
```

Values change depending on movement near the WiFi source.

---

# Hardware Used

* Laptop WiFi adapter (Realtek RTL8723DE)
* Phone hotspot used as WiFi access point

No additional sensors are required.

---

# Future Work

Planned improvements:

* real-time RF dashboard
* motion classification
* WiFi radar heatmaps
* gesture detection
* multi-person disturbance detection
* netlink-based RSSI acquisition (instead of spawning `iw` processes)

---

# Why Rust?

Rust is used for the core engine because it provides:

* high performance
* memory safety
* efficient concurrency
* predictable latency for signal processing

Python remains ideal for:

* experimentation
* visualization
* rapid prototyping

---

# Disclaimer

This project is an experimental exploration of **RF sensing using commodity WiFi hardware**.
RSSI-based sensing has limitations and may be noisy depending on hardware and environment.

---

# License

MIT License

````

---

# requirements.txt

```txt
numpy
scipy
matplotlib
maturin
````

Optional (for nicer plots later):

```txt
plotly
```

---

# Optional (Very Good Idea)

Add a **project banner image** at the top of README later like:

```
WiFi RSSI → Rust RF Engine → Motion Detection
```

This makes the repo look **much more professional**.

---
