use std::process::Command;
use regex::Regex;

const MONITOR_REGEX: &str = "(.+) connected";
const BAR_NAME: &str = "main";

fn main() {
    // Kill polybar beforehand
    Command::new("killall")
        .args(&["-vw", "polybar"])
        .output()
        .expect("Failed to kill polybar");

    // Getting all xrandr outputs
    let xrandr_resp = Command::new("xrandr")
        .output()
        .expect("Failed to call xrandr");
    let xrandr_resp = String::from_utf8_lossy(&xrandr_resp.stdout);

    // Regexing monitor names out
    
    let regex = Regex::new(MONITOR_REGEX).unwrap();
    for cap in regex.captures_iter(&xrandr_resp) {
        // Launching polybar an passing the monitor name as an env
        Command::new("polybar")
            .env("MONITOR", &cap[1])
            .args(&[BAR_NAME])
            .spawn()
            .expect("Failed to launch polybar");
    }
}
