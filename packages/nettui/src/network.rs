use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::OnceLock;

use tuigreat::AppResult;

static USE_SUDO: OnceLock<bool> = OnceLock::new();

/// Find the sudo binary. NixOS requires the wrapped sudo with setuid bit,
/// but we try the standard `sudo` from PATH first for portability.
fn find_sudo() -> &'static str {
    use std::sync::OnceLock;
    static SUDO_PATH: OnceLock<&'static str> = OnceLock::new();
    SUDO_PATH.get_or_init(|| {
        if tuigreat::which("sudo") {
            "sudo"
        } else if std::path::Path::new("/run/wrappers/bin/sudo").exists() {
            "/run/wrappers/bin/sudo"
        } else {
            "sudo" // fallback, will error naturally if missing
        }
    })
}

/// Wi-Fi backend selection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WifiBackend {
    Iwd,
    Wpa,
    Nmcli,
}

impl std::fmt::Display for WifiBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Iwd => write!(f, "iwd"),
            Self::Wpa => write!(f, "wpa"),
            Self::Nmcli => write!(f, "nmcli"),
        }
    }
}

/// Detect available Wi-Fi backend, in preference order.
pub fn detect_wifi_backend() -> Option<WifiBackend> {
    if tuigreat::which("iwctl") {
        Some(WifiBackend::Iwd)
    } else if tuigreat::which("wpa_cli") {
        Some(WifiBackend::Wpa)
    } else if tuigreat::which("nmcli") {
        Some(WifiBackend::Nmcli)
    } else {
        None
    }
}

pub fn needs_sudo() -> bool {
    *USE_SUDO.get_or_init(|| unsafe { libc::geteuid() != 0 })
}

/// Check if we have cached sudo credentials (no password needed)
pub fn has_sudo_cached() -> bool {
    if !needs_sudo() {
        return true;
    }
    Command::new(find_sudo())
        .args(["--non-interactive", "true"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Run a command with sudo, using provided password if needed
pub fn run_with_sudo(program: &str, args: &[&str], password: Option<&str>) -> AppResult<String> {
    if !needs_sudo() {
        let output = Command::new(program).args(args).output()?;
        return Ok(String::from_utf8_lossy(&output.stdout).to_string());
    }

    // Try non-interactive first (cached credentials)
    let try_cached = Command::new(find_sudo())
        .args(["--non-interactive", program])
        .args(args)
        .output()?;

    if try_cached.status.success() {
        return Ok(String::from_utf8_lossy(&try_cached.stdout).to_string());
    }

    // Need password
    let Some(pass) = password else {
        return Err("Sudo password required".into());
    };

    let mut child = Command::new(find_sudo())
        .args(["-S", program])
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "{pass}")?;
    }

    let output = child.wait_with_output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("incorrect password") || stderr.contains("Sorry") {
            Err("Incorrect password".into())
        } else {
            Err(stderr.trim().to_string().into())
        }
    }
}

#[derive(Clone)]
pub struct Interface {
    pub name: String,
    pub itype: String,
    pub oper_state: String,
    pub address: Option<String>,
}

#[derive(Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: u8,
    pub secured: bool,
    pub connected: bool,
}

pub fn get_interfaces() -> AppResult<Vec<Interface>> {
    let output = Command::new("networkctl")
        .args(["--json=short", "list"])
        .output()?;

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).unwrap_or(serde_json::json!({"Interfaces": []}));

    let interfaces = json["Interfaces"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|iface| {
                    let name = iface["Name"].as_str()?.to_string();

                    // Skip loopback
                    if name == "lo" {
                        return None;
                    }

                    let itype = iface["Type"].as_str().unwrap_or("unknown").to_string();
                    let oper_state = iface["OperationalState"]
                        .as_str()
                        .unwrap_or("off")
                        .to_string();

                    // Get IPv4 address from Addresses array
                    let address = iface["Addresses"].as_array().and_then(|addrs| {
                        addrs.iter().find_map(|a| {
                            let bytes = a["Address"].as_array()?;
                            if bytes.len() == 4 {
                                let ip: Vec<String> = bytes
                                    .iter()
                                    .filter_map(|b| b.as_u64().map(|n| n.to_string()))
                                    .collect();
                                if ip.len() == 4 {
                                    return Some(ip.join("."));
                                }
                            }
                            None
                        })
                    });

                    Some(Interface {
                        name,
                        itype,
                        oper_state,
                        address,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(interfaces)
}

pub fn toggle_interface(
    interface: &str,
    bring_up: bool,
    password: Option<&str>,
) -> AppResult<String> {
    let action = if bring_up { "up" } else { "down" };

    match run_with_sudo("networkctl", &[action, interface], password) {
        Ok(_) => Ok(format!("{interface} {action}")),
        Err(e) => {
            let msg = e.to_string();
            if msg == "Sudo password required" {
                Err(e)
            } else {
                Ok(msg)
            }
        }
    }
}
