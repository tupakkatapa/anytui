use std::process::Command;
use tuigreat::AppResult;

use crate::network::WifiNetwork;

/// Ticks to wait after triggering scan before fetching results (~2000ms).
pub const SCAN_DELAY_TICKS: u32 = 20;

/// Trigger a Wi-Fi scan (non-blocking, returns immediately).
pub fn trigger_scan(interface: &str) {
    let _ = Command::new("wpa_cli")
        .args(["-i", interface, "scan"])
        .output();
}

/// Fetch scan results (non-blocking, reads cached results from `wpa_supplicant`).
pub fn get_networks(interface: &str) -> AppResult<Vec<WifiNetwork>> {
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

/// Check if `wpa_supplicant` has a configured network for this SSID.
pub fn has_stored_credentials(interface: &str, ssid: &str) -> bool {
    find_configured_network(interface, ssid).is_some()
}

pub fn connect_wifi(interface: &str, ssid: &str, password: Option<&str>) -> AppResult<String> {
    // Without a password, try stored credentials first, then open network.
    let Some(psk) = password else {
        if let Some(net_id) = find_configured_network(interface, ssid) {
            let _ = Command::new("wpa_cli")
                .args(["-i", interface, "select_network", &net_id])
                .output()?;
            return Ok(format!("Connecting to {ssid}"));
        }
        // No configured network — connect as open (key_mgmt=NONE)
        return connect_open_network(interface, ssid);
    };

    if psk.is_empty() {
        return Ok(format!("'{ssid}' requires a password"));
    }

    // Add network with password
    let output = Command::new("wpa_cli")
        .args(["-i", interface, "add_network"])
        .output()?;
    let net_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

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

    let _ = Command::new("wpa_cli")
        .args(["-i", interface, "select_network", &net_id])
        .output()?;

    Ok(format!("Connecting to {ssid}"))
}

/// Connect to an open network (no password, `key_mgmt=NONE`).
fn connect_open_network(interface: &str, ssid: &str) -> AppResult<String> {
    let output = Command::new("wpa_cli")
        .args(["-i", interface, "add_network"])
        .output()?;
    let net_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

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

    let _ = Command::new("wpa_cli")
        .args(["-i", interface, "set_network", &net_id, "key_mgmt", "NONE"])
        .output()?;

    let _ = Command::new("wpa_cli")
        .args(["-i", interface, "select_network", &net_id])
        .output()?;

    Ok(format!("Connecting to {ssid}"))
}

/// Find an already-configured network by SSID in `wpa_supplicant`.
fn find_configured_network(interface: &str, ssid: &str) -> Option<String> {
    let output = Command::new("wpa_cli")
        .args(["-i", interface, "list_networks"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 2 && parts[1] == ssid {
            return Some(parts[0].to_string());
        }
    }
    None
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
