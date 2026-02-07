pub mod iwd;
pub mod nmcli;
pub mod wpa;

use tuigreat::AppResult;

use crate::network::{WifiBackend, WifiNetwork};

/// Scan for Wi-Fi networks using the appropriate backend.
pub fn scan_wifi(backend: WifiBackend, interface: &str) -> AppResult<Vec<WifiNetwork>> {
    match backend {
        WifiBackend::Iwd => iwd::scan_wifi(interface),
        WifiBackend::Wpa => wpa::scan_wifi(interface),
        WifiBackend::Nmcli => nmcli::scan_wifi(interface),
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
