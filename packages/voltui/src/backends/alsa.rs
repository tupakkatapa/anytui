use std::process::Command;

use crate::audio::{Sink, Source};
use tuigreat::AppResult;

struct ControlInfo {
    name: String,
    volume: u8,
    muted: bool,
}

/// Parse amixer scontents output into controls.
fn parse_alsa_controls(output: &str, playback: bool) -> Vec<ControlInfo> {
    let mut controls = Vec::new();
    let mut current_name = String::new();
    let mut current_volume: Option<u8> = None;
    let mut current_muted = false;
    let mut is_relevant = false;

    for line in output.lines() {
        if let Some(rest) = line.strip_prefix("Simple mixer control '") {
            // Save previous control if relevant
            if is_relevant && !current_name.is_empty() {
                controls.push(ControlInfo {
                    name: current_name.clone(),
                    volume: current_volume.unwrap_or(0),
                    muted: current_muted,
                });
            }
            // Parse new control name
            current_name = rest.split('\'').next().unwrap_or("").to_string();
            current_volume = None;
            current_muted = false;
            is_relevant = false;
        } else if line.contains("Capabilities:") {
            let caps = line.trim();
            if playback {
                is_relevant = caps.contains("pvolume") || caps.contains("pswitch");
            } else {
                is_relevant = caps.contains("cvolume") || caps.contains("cswitch");
            }
        } else if is_relevant {
            // Look for volume percentage [XX%]
            if let Some(pct_start) = line.find('[')
                && let Some(pct_end) = line[pct_start + 1..].find('%')
                && let Ok(vol) = line[pct_start + 1..pct_start + 1 + pct_end].parse::<u8>()
                && current_volume.is_none()
            {
                current_volume = Some(vol);
            }
            // Look for mute state [on]/[off]
            if line.contains("[off]") {
                current_muted = true;
            }
        }
    }

    // Don't forget the last control
    if is_relevant && !current_name.is_empty() {
        controls.push(ControlInfo {
            name: current_name,
            volume: current_volume.unwrap_or(0),
            muted: current_muted,
        });
    }

    controls
}

pub fn get_sinks() -> AppResult<Vec<Sink>> {
    let output = Command::new("amixer").args(["scontents"]).output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let controls = parse_alsa_controls(&text, true);

    Ok(controls
        .into_iter()
        .map(|c| Sink {
            is_default: c.name == "Master",
            description: c.name.clone(),
            name: c.name,
            volume: c.volume,
            muted: c.muted,
        })
        .collect())
}

pub fn get_sources() -> AppResult<Vec<Source>> {
    let output = Command::new("amixer").args(["scontents"]).output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let controls = parse_alsa_controls(&text, false);

    Ok(controls
        .into_iter()
        .map(|c| Source {
            is_default: c.name == "Capture",
            description: c.name.clone(),
            name: c.name,
            volume: c.volume,
            muted: c.muted,
        })
        .collect())
}

pub fn adjust_sink_volume(name: &str, delta: i8) -> AppResult<()> {
    let change = if delta > 0 { "5%+" } else { "5%-" };
    Command::new("amixer")
        .args(["sset", name, change])
        .output()?;
    Ok(())
}

pub fn adjust_source_volume(name: &str, delta: i8) -> AppResult<()> {
    let change = if delta > 0 { "5%+" } else { "5%-" };
    Command::new("amixer")
        .args(["sset", name, change])
        .output()?;
    Ok(())
}

pub fn toggle_sink_mute(name: &str) -> AppResult<()> {
    Command::new("amixer")
        .args(["sset", name, "toggle"])
        .output()?;
    Ok(())
}

pub fn toggle_source_mute(name: &str) -> AppResult<()> {
    Command::new("amixer")
        .args(["sset", name, "toggle"])
        .output()?;
    Ok(())
}
