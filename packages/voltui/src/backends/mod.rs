pub mod alsa;
pub mod pactl;

use crate::audio::{AudioBackend, Sink, Source};
use tuigreat::AppResult;

pub fn get_sinks(backend: AudioBackend) -> AppResult<Vec<Sink>> {
    match backend {
        AudioBackend::PulseAudio => pactl::get_sinks(),
        AudioBackend::Alsa => alsa::get_sinks(),
    }
}

pub fn get_sources(backend: AudioBackend) -> AppResult<Vec<Source>> {
    match backend {
        AudioBackend::PulseAudio => pactl::get_sources(),
        AudioBackend::Alsa => alsa::get_sources(),
    }
}

pub fn adjust_sink_volume(backend: AudioBackend, name: &str, delta: i8) -> AppResult<()> {
    match backend {
        AudioBackend::PulseAudio => pactl::adjust_sink_volume(name, delta),
        AudioBackend::Alsa => alsa::adjust_sink_volume(name, delta),
    }
}

pub fn adjust_source_volume(backend: AudioBackend, name: &str, delta: i8) -> AppResult<()> {
    match backend {
        AudioBackend::PulseAudio => pactl::adjust_source_volume(name, delta),
        AudioBackend::Alsa => alsa::adjust_source_volume(name, delta),
    }
}

pub fn toggle_sink_mute(backend: AudioBackend, name: &str) -> AppResult<()> {
    match backend {
        AudioBackend::PulseAudio => pactl::toggle_sink_mute(name),
        AudioBackend::Alsa => alsa::toggle_sink_mute(name),
    }
}

pub fn toggle_source_mute(backend: AudioBackend, name: &str) -> AppResult<()> {
    match backend {
        AudioBackend::PulseAudio => pactl::toggle_source_mute(name),
        AudioBackend::Alsa => alsa::toggle_source_mute(name),
    }
}

pub fn set_default_sink(backend: AudioBackend, name: &str) -> Result<(), String> {
    match backend {
        AudioBackend::PulseAudio => pactl::set_default_sink(name),
        AudioBackend::Alsa => Err("ALSA does not support setting default sink".to_string()),
    }
}

pub fn set_default_source(backend: AudioBackend, name: &str) -> Result<(), String> {
    match backend {
        AudioBackend::PulseAudio => pactl::set_default_source(name),
        AudioBackend::Alsa => Err("ALSA does not support setting default source".to_string()),
    }
}
