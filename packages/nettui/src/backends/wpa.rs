use std::process::Command;
use tuigreat::AppResult;

use crate::network::WifiNetwork;

pub fn scan_wifi(interface: &str) -> AppResult<Vec<WifiNetwork>> {
    // Trigger scan
    let _ = Command::new("wpa_cli")
        .args(["-i", interface, "scan"])
        .output();

    std::thread::sleep(std::time::Duration::from_millis(2000));

    let output = Command::new("wpa_cli")
        .args(["-i", interface, "scan_results"])
        .output()?;

    if output.status.success() {
        let connected_ssid = get_connected_network(interface);
        Ok(parse_wpa_scan_results(
            &String::from_utf8_lossy(&output.stdout),
            connected_ssid.as_deref(),
        ))
    } else {
        Ok(vec![])
    }
}

pub fn connect_wifi(interface: &str, ssid: &str, password: Option<&str>) -> AppResult<String> {
    // Add network
    let output = Command::new("wpa_cli")
        .args(["-i", interface, "add_network"])
        .output()?;
    let net_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Set SSID
    let _ = Command::new("wpa_cli")
        .args([
            "-i",
            interface,
            "set_network",
            &net_id,
            "ssid",
            &format!("\"{ssid}\""),
        ])
        .output()?;

    // Set password or open
    if let Some(psk) = password {
        if psk.is_empty() {
            return Ok(format!("'{ssid}' requires a password"));
        }
        let _ = Command::new("wpa_cli")
            .args([
                "-i",
                interface,
                "set_network",
                &net_id,
                "psk",
                &format!("\"{psk}\""),
            ])
            .output()?;
    } else {
        let _ = Command::new("wpa_cli")
            .args(["-i", interface, "set_network", &net_id, "key_mgmt", "NONE"])
            .output()?;
    }

    // Enable and select
    let _ = Command::new("wpa_cli")
        .args(["-i", interface, "select_network", &net_id])
        .output()?;

    Ok(format!("Connecting to {ssid}"))
}

fn get_connected_network(interface: &str) -> Option<String> {
    let output = Command::new("wpa_cli")
        .args(["-i", interface, "status"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(ssid) = line.strip_prefix("ssid=") {
            return Some(ssid.to_string());
        }
    }
    None
}

/// Parse `wpa_cli` `scan_results` output.
/// Format:
/// ```
/// bssid / frequency / signal level / flags / ssid
/// aa:bb:cc:dd:ee:ff    2437    -45    [WPA2-PSK-CCMP][ESS]    MyNetwork
/// ```
fn parse_wpa_scan_results(output: &str, connected: Option<&str>) -> Vec<WifiNetwork> {
    let mut networks = Vec::new();

    for line in output.lines().skip(1) {
        // Skip header line
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 5 {
            continue;
        }

        let signal_dbm: i32 = parts[2].parse().unwrap_or(-100);
        // Convert dBm to percentage (rough approximation)
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let signal = ((signal_dbm + 100).clamp(0, 100) * 100 / 70).clamp(0, 100) as u8;

        let flags = parts[3];
        let secured = flags.contains("WPA") || flags.contains("WEP");
        let ssid = parts[4..].join("\t");

        if ssid.is_empty() {
            continue;
        }

        let is_connected = connected.is_some_and(|c| c == ssid);

        networks.push(WifiNetwork {
            ssid,
            signal,
            secured,
            connected: is_connected,
        });
    }

    networks
}
