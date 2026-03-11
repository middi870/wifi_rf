import matplotlib
matplotlib.use("TkAgg")

import numpy as np
import matplotlib.pyplot as plt
import time
from collections import deque

import wifi_core
from rssi_reader import get_rssi


BUFFER = 300

signal_buffer = deque(maxlen=BUFFER)
motion_map = deque(maxlen=100)


plt.ion()
fig, ax = plt.subplots()


while plt.fignum_exists(fig.number):

    rssi = get_rssi()

    if rssi is not None:
        signal_buffer.append(rssi)

    if len(signal_buffer) < 50:
        continue

    data = np.array(signal_buffer)

    # Rust filters
    butter = wifi_core.butterworth(data.tolist(), 0.2)
    kalman = wifi_core.kalman(butter, 0.01, 2.0)
    filtered = wifi_core.ewma(kalman, 0.3)

    motion = np.var(filtered[-20:])

    motion_map.append(motion)

    heat = np.array(motion_map).reshape(1,-1)

    ax.clear()
    ax.set_title("WiFi Motion Radar")

    ax.imshow(heat, aspect="auto")

    plt.pause(0.01)

    time.sleep(0.1)