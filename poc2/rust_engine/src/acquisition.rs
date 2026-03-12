use regex::Regex;
use std::process::Command;

pub fn read_rssi(interface: &str) -> Option<f64> {
    let output = Command::new("iw")
    .args(["dev", interface, "link"])
    .output()
    .ok()?;

    let text = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"signal:\s*(-\d+)").ok()?;

    if let Some(cap) = re.captures(&text) {
        return cap.get(1)?.as_str().parse().ok();
    }
    None
}