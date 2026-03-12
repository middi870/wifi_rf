import rf_engine
import time

while True:

    rssi = rf_engine.get_rssi("wlan0")

    print("RSSI:", rssi)

    time.sleep(0.5)