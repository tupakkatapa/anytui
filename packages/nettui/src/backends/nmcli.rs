use std::process::Command;
use tuigreat::AppResult;

use crate::network::WifiNetwork;

pub fn scan_wifi(interface: &str) -> AppResult<Vec<WifiNetwork>> {
    // Trigger rescan
    let _ = Command::new("nmcli")
        .args(["dev", "wifi", "rescan", "ifname", interface])
        .output();

    std::thread::sleep(std::time::Duration::from_millis(1000));

    let output = Command::new("nmcli")
        .args([
            "-t",
            "-f",
            "SSID,SIGNAL,SECURITY,ACTIVE",
            "dev",
            "wifi",
            "list",
            "ifname",
            interface,
        ])
        .output()?;

    if output.status.success() {
        Ok(parse_nmcli_output(&String::from_utf8_lossy(&output.stdout)))
    } else {
        Ok(vec![])
    }
}

pub fn connect_wifi(interface: &str, ssid: &str, password: Option<&str>) -> AppResult<String> {
    let mut args = vec!["dev", "wifi", "connect", ssid, "ifname", interface];
    if let Some(psk) = password {
        if psk.is_empty() {
            return Ok(format!("'{ssid}' requires a password"));
        }
        args.extend(["password", psk]);
    }

    let output = Command::new("nmcli").args(&args).output()?;

    if output.status.success() {
        Ok(format!("Connecting to {ssid}"))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("Secrets were required") || stderr.contains("No suitable") {
            Ok(format!("'{ssid}' requires a password"))
        } else if stderr.is_empty() {
            Ok(format!("Connecting to {ssid}"))
        } else {
            Ok(format!("Failed: {}", stderr.trim()))
        }
    }
}

/// Parse nmcli -t output.
/// Format: SSID:SIGNAL:SECURITY:ACTIVE (colon-separated, with \: escaping)
fn parse_nmcli_output(output: &str) -> Vec<WifiNetwork> {
    let mut networks = Vec::new();

    for line in output.lines() {
        let fields = split_escaped(line, ':');
        if fields.len() < 4 {
            continue;
        }

        let ssid = fields[0].clone();
        if ssid.is_empty() {
            continue;
        }

        let signal: u8 = fields[1].parse().unwrap_or(0);
        let secured = !fields[2].is_empty() && fields[2] != "--";
        let connected = fields[3] == "yes";

        networks.push(WifiNetwork {
            ssid,
            signal,
            secured,
            connected,
        });
    }

    networks
}

/// Split a string by delimiter, respecting backslash escaping.
fn split_escaped(s: &str, delimiter: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut escape = false;

    for c in s.chars() {
        if escape {
            current.push(c);
            escape = false;
        } else if c == '\\' {
            escape = true;
        } else if c == delimiter {
            fields.push(current.clone());
            current.clear();
        } else {
            current.push(c);
        }
    }
    fields.push(current);
    fields
}
