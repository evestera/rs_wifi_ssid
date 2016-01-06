extern crate regex;

use std::process::Command;
use std::str;

use self::regex::Regex;

/// Attempts to get the SSID of the currently connected network
pub fn current() -> Option<String> {
    let stdout = match Command::new("netsh")
                           .arg("wlan")
                           .arg("show")
                           .arg("interface")
                           .output() {
        Ok(out) => out.stdout,
        Err(_) => return None,
    };

    let s = match str::from_utf8(&stdout) {
        Ok(s) => s,
        Err(_) => return None,
    };

    Regex::new(r"SSID\s*: (.+?)\r\n")
        .unwrap()
        .captures(s)
        .map(|caps| (&caps[1]).to_string())
}

/// Returns a Vec<String> with SSIDs of networks in range.
///
/// Performs a scan and blocks while scanning.
/// Returns an empty Vec on failure.
pub fn list_all() -> Vec<String> {
    let stdout = match Command::new("netsh")
                           .arg("wlan")
                           .arg("show")
                           .arg("networks")
                           .output() {
        Ok(out) => out.stdout,
        Err(_) => return vec![],
    };

    let s = match str::from_utf8(&stdout) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    Regex::new(r"SSID \d+ : (.+?)\r\n")
        .unwrap()
        .captures_iter(s)
        .map(|caps| (&caps[1]).to_string())
        .collect()
}
