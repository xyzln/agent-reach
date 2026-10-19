use crate::channels::Channel;

/// Health status of a channel's active backend, as probed by `doctor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok,
    Missing,
    Broken,
    Timeout,
}

/// A resolved (channel, backend) pair.
#[derive(Debug, Clone)]
pub struct Selection {
    pub channel: Channel,
    pub backend: String,
}

/// Choose the active backend for a channel.
///
/// Mirrors the original ordered-backend-candidates design: each channel carries
/// a preferred backend, but it can be overridden per-channel via an environment
/// variable `<CHANNEL>_BACKEND` (surfaced here as the `override_backend` arg).
#[allow(dead_code)]
pub fn select(channel: &Channel, override_backend: Option<&str>) -> Selection {
    let backend = match override_backend {
        Some(b) => b.to_string(),
        None => channel.preferred_backend.to_string(),
    };
    Selection {
        channel: channel.clone(),
        backend,
    }
}

/// Full registry of supported channels.
#[allow(dead_code)]
pub fn registry() -> Vec<Channel> {
    crate::channels::all()
}
