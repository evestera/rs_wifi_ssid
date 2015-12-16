use std::process::Command;

/// Attempts to get the SSID of the currently connected network
pub fn current() -> Option<String> {
    let stdout = match Command::new("iwgetid").arg("--raw").output() {
        Ok(out) => out.stdout,
        Err(_) => return None,
    };

    if stdout.is_empty() {
        return None;
    }

    match String::from_utf8(stdout) {
        Ok(s) => Some(s.replace("\n", "")),
        Err(_) => None,
    }
}

/// Returns a Vec<String> with SSIDs of networks in range.
///
/// Not yet implemented
pub fn list_all() -> Vec<String> {
    unimplemented!();
}
