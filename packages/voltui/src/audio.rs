/// Audio backend selection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioBackend {
    PulseAudio,
    Alsa,
}

impl std::fmt::Display for AudioBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PulseAudio => write!(f, "pipewire"),
            Self::Alsa => write!(f, "alsa"),
        }
    }
}

/// Detect available audio backend, in preference order.
pub fn detect_audio_backend() -> Option<AudioBackend> {
    if tuigreat::which("pactl") {
        Some(AudioBackend::PulseAudio)
    } else if tuigreat::which("amixer") {
        Some(AudioBackend::Alsa)
    } else {
        None
    }
}

#[derive(Clone)]
pub struct Sink {
    pub name: String,
    pub description: String,
    pub volume: u8,
    pub muted: bool,
    pub is_default: bool,
}

#[derive(Clone)]
pub struct Source {
    pub name: String,
    pub description: String,
    pub volume: u8,
    pub muted: bool,
    pub is_default: bool,
}
