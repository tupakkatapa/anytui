use std::process::Command;
use tuigreat::AppResult;

use crate::network::WifiNetwork;

pub fn scan_wifi(interface: &str) -> AppResult<Vec<WifiNetwork>> {
    // Trigger scan (iwd doesn't require sudo)
    let _ = Command::new("iwctl")
        .args(["station", interface, "scan"])
        .output();

    // Small delay to let scan complete
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Get results
    let output = Command::new("iwctl")
        .args(["station", interface, "get-networks"])
        .output()?;

    if output.status.success() {
        let connected = get_connected_network(interface);
        Ok(parse_iwctl_networks(
            &String::from_utf8_lossy(&output.stdout),
            connected.as_deref(),
        ))
    } else {
        Ok(vec![])
    }
}

pub fn connect_wifi(interface: &str, ssid: &str, password: Option<&str>) -> AppResult<String> {
    let output = if let Some(psk) = password {
        if psk.is_empty() {
            return Ok(format!("'{ssid}' requires a password"));
        }
        Command::new("iwctl")
            .args(["--passphrase", psk, "station", interface, "connect", ssid])
            .output()?
    } else {
        Command::new("iwctl")
            .args(["station", interface, "connect", ssid])
            .output()?
    };

    if output.status.success() {
        Ok(format!("Connecting to {ssid}"))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("passphrase") || stderr.contains("Passphrase") {
            Ok(format!("'{ssid}' requires a password"))
        } else if stderr.is_empty() {
            Ok(format!("Connecting to {ssid}"))
        } else {
            Ok(format!("Failed: {}", stderr.trim()))
        }
    }
}

/// Get currently connected network from iwctl station show.
fn get_connected_network(interface: &str) -> Option<String> {
    let output = Command::new("iwctl")
        .args(["station", interface, "show"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Connected network") {
            let parts: Vec<&str> = trimmed.splitn(2, "Connected network").collect();
            if parts.len() == 2 {
                return Some(parts[1].trim().to_string());
            }
        }
    }
    None
}

/// Parse iwctl get-networks output.
fn parse_iwctl_networks(output: &str, connected: Option<&str>) -> Vec<WifiNetwork> {
    let mut networks = Vec::new();
    let mut in_data = false;
    let mut header_count = 0;

    for line in output.lines() {
        if line.contains("----") {
            header_count += 1;
            if header_count >= 2 {
                in_data = true;
            }
            continue;
        }

        if !in_data {
            continue;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let is_connected_indicator = line.trim_start().starts_with('>');
        let line_clean = if is_connected_indicator {
            line.trim_start().trim_start_matches('>').trim_start()
        } else {
            trimmed
        };

        // Find signal strength (asterisks at end)
        let signal_chars: String = line_clean
            .chars()
            .rev()
            .take_while(|c| *c == '*' || c.is_whitespace())
            .collect::<String>()
            .chars()
            .rev()
            .collect();
        let signal_count = signal_chars.chars().filter(|c| *c == '*').count();
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::cast_precision_loss
        )]
        let signal = ((signal_count as f32 / 4.0) * 100.0) as u8;

        let without_signal = line_clean.trim_end_matches(|c: char| c == '*' || c.is_whitespace());
        let parts: Vec<&str> = without_signal.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let security = parts.last().unwrap_or(&"");
        let secured = *security != "open";
        let ssid = parts[..parts.len() - 1].join(" ");

        if ssid.is_empty() || ssid == "Network name" {
            continue;
        }

        let is_connected = connected.is_some_and(|c| c == ssid) || is_connected_indicator;

        networks.push(WifiNetwork {
            ssid,
            signal,
            secured,
            connected: is_connected,
        });
    }

    networks
}
