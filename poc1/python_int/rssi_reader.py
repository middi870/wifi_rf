import subprocess
import re
import time

INTERFACE = "wlan0" #Change this after finding you device name using: iw dev

def get_rssi():
    try:
        output = subprocess.check_output(
            ["iw", "dev", INTERFACE, "link"]
            ).decode()

        match = re.search(r"signal:\s*(-\d+)", output)

        if match:
            return int(match.group(1))
    
    except Exception:
        return None

    return None

if __name__ == "__main__":
    
    while True:
        rssi = get_rssi()

        if rssi is not None:
            print(f"RSSI: {rssi}")

        time.sleep(0.2)