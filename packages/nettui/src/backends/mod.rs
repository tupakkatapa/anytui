pub mod iwd;
pub mod nmcli;
pub mod wpa;

use tuigreat::AppResult;

use crate::network::{WifiBackend, WifiNetwork};

/// Trigger a Wi-Fi scan (non-blocking).
pub fn trigger_scan(backend: WifiBackend, interface: &str) {
    match backend {
        WifiBackend::Iwd => iwd::trigger_scan(interface),
        WifiBackend::Wpa => wpa::trigger_scan(interface),
        WifiBackend::Nmcli => nmcli::trigger_scan(interface),
    }
}

/// Fetch scan results (non-blocking, reads cached results).
pub fn get_networks(backend: WifiBackend, interface: &str) -> AppResult<Vec<WifiNetwork>> {
    match backend {
        WifiBackend::Iwd => iwd::get_networks(interface),
        WifiBackend::Wpa => wpa::get_networks(interface),
        WifiBackend::Nmcli => nmcli::get_networks(interface),
    }
}

/// Ticks to wait between scan trigger and result fetch.
pub fn scan_delay_ticks(backend: WifiBackend) -> u32 {
    match backend {
        WifiBackend::Iwd => iwd::SCAN_DELAY_TICKS,
        WifiBackend::Wpa => wpa::SCAN_DELAY_TICKS,
        WifiBackend::Nmcli => nmcli::SCAN_DELAY_TICKS,
    }
}

/// Check if the backend has stored credentials for a network.
pub fn has_stored_credentials(backend: WifiBackend, interface: &str, ssid: &str) -> bool {
    match backend {
        WifiBackend::Iwd => iwd::has_stored_credentials(ssid),
        WifiBackend::Wpa => wpa::has_stored_credentials(interface, ssid),
        WifiBackend::Nmcli => nmcli::has_stored_credentials(ssid),
    }
}

/// Connect to a Wi-Fi network using the appropriate backend.
pub fn connect_wifi(
    backend: WifiBackend,
    interface: &str,
    ssid: &str,
    password: Option<&str>,
) -> AppResult<String> {
    match backend {
        WifiBackend::Iwd => iwd::connect_wifi(interface, ssid, password),
        WifiBackend::Wpa => wpa::connect_wifi(interface, ssid, password),
        WifiBackend::Nmcli => nmcli::connect_wifi(interface, ssid, password),
    }
}
