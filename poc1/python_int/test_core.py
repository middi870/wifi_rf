import matplotlib
matplotlib.use('TkAgg')

import numpy as np
import wifi_core
import matplotlib.pyplot as plt

# generate a fake RSSI signal
signal = np.sin(np.linspace(0,20,500)) + np.random.normal(0,0.2,500)

# Rust EWMA filter
filtered = wifi_core.ewma(signal.tolist(), 0.3)

# Rust FFT
spectrum = wifi_core.fft_power(filtered)

# plot the results
plt.subplot(2,1,1)
plt.title("Signal")
plt.plot(signal)
plt.plot(filtered)

plt.subplot(2,1,2)
plt.title("Spectrum")
plt.plot(spectrum)

plt.show()

