import matplotlib
matplotlib.use("TkAgg")

import numpy as np
import matplotlib.pyplot as plt
import time
from collections import deque

import wifi_core
from rssi_reader import get_rssi


BUFFER = 512

signal_buffer = deque(maxlen=BUFFER)


plt.ion()
fig, (ax1, ax2) = plt.subplots(2,1)


while True:

    rssi = get_rssi()

    if rssi is not None:
        signal_buffer.append(rssi)

    if len(signal_buffer) < BUFFER:
        continue

    data = np.array(signal_buffer)

    butter = wifi_core.butterworth(data.tolist(),0.2)
    kalman = wifi_core.kalman(butter,0.01,2.0)
    filtered = wifi_core.ewma(kalman,0.3)

    spectrum = wifi_core.fft_power(filtered)

    spectrum = np.array(spectrum)

    freqs = np.fft.fftfreq(len(spectrum), d=0.1)

    mask = (freqs > 0.1) & (freqs < 0.5)

    breathing_freq = freqs[mask][np.argmax(spectrum[mask])]

    bpm = breathing_freq * 60


    ax1.clear()
    ax1.set_title("Filtered RSSI Signal")
    ax1.plot(filtered)

    ax2.clear()
    ax2.set_title(f"Estimated Breathing Rate: {bpm:.1f} BPM")
    ax2.plot(freqs, spectrum)

    plt.pause(0.01)

    time.sleep(0.1)