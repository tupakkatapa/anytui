pub mod app;
pub mod clipboard;
pub mod keys;
pub mod status;
pub mod theme;
pub mod widgets;

pub use app::{App, AppResult};
pub use clipboard::{paste, yank};
pub use keys::{Action, KeyHandler};
pub use status::{StatusLevel, StatusMessage, status_line};
pub use theme::Theme;

use std::process::{Command, Stdio};

/// Check if a command is available on the system PATH.
#[must_use]
pub fn which(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
