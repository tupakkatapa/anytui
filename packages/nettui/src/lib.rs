//! Network TUI parsing utilities.

/// A Wi-Fi network discovered during scanning.
#[derive(Clone, Debug, PartialEq)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: u8,
    pub secured: bool,
    pub connected: bool,
}

/// Split a string by delimiter, respecting backslash escaping.
#[must_use]
pub fn split_escaped(s: &str, delimiter: char) -> Vec<String> {
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

/// Parse nmcli -t output.
/// Format: SSID:SIGNAL:SECURITY:ACTIVE (colon-separated, with \: escaping)
#[must_use]
pub fn parse_nmcli_output(output: &str) -> Vec<WifiNetwork> {
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

/// Parse `wpa_cli` `scan_results` output.
/// Format: `bssid\tfrequency\tsignal_level\tflags\tssid`
#[must_use]
pub fn parse_wpa_scan_results(output: &str, connected: Option<&str>) -> Vec<WifiNetwork> {
    let mut networks = Vec::new();

    for line in output.lines().skip(1) {
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

/// Parse iwctl network list output.
/// Signal strength is shown as asterisks (* to ****).
#[must_use]
pub fn parse_iwctl_networks(output: &str, connected: Option<&str>) -> Vec<WifiNetwork> {
    let mut networks = Vec::new();

    for line in output.lines() {
        // Skip headers and separators
        if line.contains("---") || line.trim().is_empty() || line.contains("Network name") {
            continue;
        }

        // Connected networks start with '>'
        let is_connected_line = line.starts_with('>');
        let line = line.trim_start_matches('>').trim();

        // Parse: "SSID                Security   Signal"
        // Signal is shown as asterisks
        let asterisk_count = line.matches('*').count();
        #[allow(clippy::cast_possible_truncation)]
        let signal = ((asterisk_count * 25).min(100)) as u8;

        // Extract SSID (everything before the security column)
        // Find where asterisks start and work backwards
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        // Last parts are signal (asterisks) and security
        // SSID is everything else
        let mut ssid_parts = Vec::new();
        let mut found_security = false;
        for part in &parts {
            if *part == "open" || *part == "psk" || *part == "8021x" || part.contains('*') {
                found_security = true;
                continue;
            }
            if !found_security {
                ssid_parts.push(*part);
            }
        }

        let ssid = ssid_parts.join(" ");
        if ssid.is_empty() {
            continue;
        }

        let secured = !line.contains("open");
        let is_connected = is_connected_line || connected.is_some_and(|c| c == ssid);

        networks.push(WifiNetwork {
            ssid,
            signal,
            secured,
            connected: is_connected,
        });
    }

    networks
}

/// Convert dBm signal strength to percentage (0-100).
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn dbm_to_percent(dbm: i32) -> u8 {
    ((dbm + 100).clamp(0, 100) * 100 / 70).clamp(0, 100) as u8
}
