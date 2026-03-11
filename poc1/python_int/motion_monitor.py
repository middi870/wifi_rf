from time import time
from scipy.signal import butter, filtfilt

import matplotlib
matplotlib.use("TkAgg")

import matplotlib.pyplot as plt

import numpy as np 
import time

from collections import deque

import wifi_core
from rssi_reader import get_rssi

BUFFER_SIZE = 200
MOTION_THRESHOLD = 2.0

buffer = deque(maxlen=BUFFER_SIZE)

plt.ion()

fig, (ax1, ax2) = plt.subplots(2, 1)

def motion_score(signal):
    if len(signal) < 20:
        return 0
    return np.var(signal[-20:])

def butter_lowpass(signal):
    b, a = butter(3, 0.15)
    return filtfilt(b, a, signal)

last_motion_state = False

while True:
    rssi = get_rssi()

    if rssi is not None:
        buffer.append(rssi)

    if len(buffer) < 50:
        continue

    data = np.array(buffer)

    buttered = butter_lowpass(data)

    filtered = wifi_core.ewma(buttered.tolist(), 0.3)

    score = motion_score(filtered)
    motion = score > MOTION_THRESHOLD

    # print only on state change
    if motion and not last_motion_state:
        print("⚠ Motion detected")

    if not motion and last_motion_state:
        print("✓ Motion stopped")

    last_motion_state = motion

    ax1.clear()
    ax1.set_title("WiFi RSSI Signal")

    ax1.plot(data, label="raw")
    ax1.plot(filtered, label="filtered")
    ax1.legend()

    ax2.clear()

    if motion:
        ax2.set_title(f"MOTION DETECTED score={score:.2f}")
        ax2.bar(["motion"], [score], color="red")

    else:
        ax2.set_title(f"NO MOTION score={score:.2f}")
        ax2.bar(["motion"], [score], color="green")

    plt.pause(0.01)
    time.sleep(0.1)