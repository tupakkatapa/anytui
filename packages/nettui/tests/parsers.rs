use nettui::{
    WifiNetwork, dbm_to_percent, parse_iwctl_networks, parse_nmcli_output, parse_wpa_scan_results,
    split_escaped,
};

// ============================================================================
// split_escaped tests
// ============================================================================

#[test]
fn test_split_escaped_simple() {
    assert_eq!(split_escaped("a:b:c", ':'), vec!["a", "b", "c"]);
}

#[test]
fn test_split_escaped_with_escape() {
    assert_eq!(split_escaped(r"a\:b:c", ':'), vec!["a:b", "c"]);
}

#[test]
fn test_split_escaped_empty_fields() {
    assert_eq!(split_escaped("a::c", ':'), vec!["a", "", "c"]);
    assert_eq!(split_escaped(":b:", ':'), vec!["", "b", ""]);
}

#[test]
fn test_split_escaped_single() {
    assert_eq!(split_escaped("single", ':'), vec!["single"]);
}

// ============================================================================
// parse_nmcli_output tests
// ============================================================================

#[test]
fn test_nmcli_basic() {
    let output = "MyNetwork:75:WPA2-PSK:no\nOpenNet:50:--:no\nConnected:80:WPA:yes";
    let networks = parse_nmcli_output(output);

    assert_eq!(networks.len(), 3);

    assert_eq!(networks[0].ssid, "MyNetwork");
    assert_eq!(networks[0].signal, 75);
    assert!(networks[0].secured);
    assert!(!networks[0].connected);

    assert_eq!(networks[1].ssid, "OpenNet");
    assert!(!networks[1].secured);

    assert_eq!(networks[2].ssid, "Connected");
    assert!(networks[2].connected);
}

#[test]
fn test_nmcli_empty_ssid_skipped() {
    let output = ":50:WPA:no\nValid:60:WPA:no";
    let networks = parse_nmcli_output(output);
    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0].ssid, "Valid");
}

#[test]
fn test_nmcli_escaped_colon() {
    let output = r"My\:Network:75:WPA:no";
    let networks = parse_nmcli_output(output);
    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0].ssid, "My:Network");
}

#[test]
fn test_nmcli_malformed_skipped() {
    let output = "TooFew:50\nValid:60:WPA:no";
    let networks = parse_nmcli_output(output);
    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0].ssid, "Valid");
}

// ============================================================================
// parse_wpa_scan_results tests
// ============================================================================

#[test]
fn test_wpa_basic() {
    let output = "bssid / frequency / signal level / flags / ssid\n\
                  aa:bb:cc:dd:ee:ff\t2437\t-45\t[WPA2-PSK-CCMP][ESS]\tMyNetwork";
    let networks = parse_wpa_scan_results(output, None);

    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0].ssid, "MyNetwork");
    assert!(networks[0].secured);
    assert!(!networks[0].connected);
}

#[test]
fn test_wpa_open_network() {
    let output = "header\naa:bb:cc:dd:ee:ff\t2437\t-50\t[ESS]\tOpenNetwork";
    let networks = parse_wpa_scan_results(output, None);

    assert_eq!(networks.len(), 1);
    assert!(!networks[0].secured);
}

#[test]
fn test_wpa_connected() {
    let output = "header\naa:bb:cc:dd:ee:ff\t2437\t-45\t[WPA2]\tMyNetwork";
    let networks = parse_wpa_scan_results(output, Some("MyNetwork"));

    assert_eq!(networks.len(), 1);
    assert!(networks[0].connected);
}

#[test]
fn test_wpa_signal_conversion() {
    // -45 dBm should give good signal, -100 dBm should give 0
    let output = "header\naa:bb\t2437\t-45\t[WPA]\tGood\nbb:cc\t2437\t-100\t[WPA]\tWeak";
    let networks = parse_wpa_scan_results(output, None);

    assert_eq!(networks.len(), 2);
    assert!(networks[0].signal > 50); // Good signal
    assert_eq!(networks[1].signal, 0); // Weak signal
}

// ============================================================================
// parse_iwctl_networks tests
// ============================================================================

#[test]
fn test_iwctl_basic() {
    let output = "                                    Network name                    Security   Signal\n\
                  ---------------------------------------------------------------------------------\n\
                  MyNetwork                                                           psk        ****";
    let networks = parse_iwctl_networks(output, None);

    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0].ssid, "MyNetwork");
    assert!(networks[0].secured);
    assert_eq!(networks[0].signal, 100); // 4 asterisks = 100%
}

#[test]
fn test_iwctl_signal_levels() {
    // Test different signal strengths
    let output = "Network name                    Security   Signal\n\
                  -------------------------------------------------\n\
                  Strong                          psk        ****\n\
                  Medium                          psk        **\n\
                  Weak                            psk        *";
    let networks = parse_iwctl_networks(output, None);

    assert_eq!(networks.len(), 3);
    assert_eq!(networks[0].signal, 100); // 4 stars
    assert_eq!(networks[1].signal, 50); // 2 stars
    assert_eq!(networks[2].signal, 25); // 1 star
}

#[test]
fn test_iwctl_open_network() {
    let output = "Network name  Security  Signal\n---\nOpenNet  open  ***";
    let networks = parse_iwctl_networks(output, None);

    assert_eq!(networks.len(), 1);
    assert!(!networks[0].secured);
}

#[test]
fn test_iwctl_connected() {
    let output = "Network name  Security  Signal\n---\n> MyNetwork  psk  ****";
    let networks = parse_iwctl_networks(output, None);

    assert_eq!(networks.len(), 1);
    assert!(networks[0].connected);
}

// ============================================================================
// dbm_to_percent tests
// ============================================================================

#[test]
fn test_dbm_conversion() {
    assert_eq!(dbm_to_percent(-100), 0);
    assert_eq!(dbm_to_percent(-30), 100);
    assert!(dbm_to_percent(-50) > 50);
    assert!(dbm_to_percent(-80) < 50);
}
